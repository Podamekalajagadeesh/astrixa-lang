use std::collections::{HashMap, HashSet};
use crate::ast::{Expr, Stmt, Function, Contract};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::ai_runtime::{AIRuntime, LocalAIRuntime};
use crate::package_manager::PackageManager;

#[derive(Clone)]
pub struct BlockchainContext {
    pub chain_id: i64,
    pub chain_name: String,
    pub sender: String,
    pub msg_value: u128,
    pub msg_data: String,
    pub tx_hash: String,
    pub tx_timestamp: i64,
}

#[derive(Clone)]
pub enum Value {
    String(String),
    Number(i64),
    Bool(bool),
    Array(Vec<Value>),
    Address(String),      // Web3: Blockchain address
    U256(u128),           // Web3: 256-bit unsigned integer
    AIResult {            // AI: Inference result
        label: String,
        score: f64,
    },
    Null,
}

#[derive(Clone)]
enum Control {
    Value(Value),
    Return(Value),
}

type ExecResult = Result<Control, String>;
type EvalResult = Result<Value, String>;

pub struct Interpreter {
    variables: HashMap<String, Value>,
    functions: HashMap<String, Function>,
    contracts: HashMap<String, Contract>,
    contract_state: HashMap<String, HashMap<String, Value>>, // contract_name -> state vars
    loaded_modules: HashSet<String>,
    blockchain_context: BlockchainContext,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            functions: HashMap::new(),
            contracts: HashMap::new(),
            contract_state: HashMap::new(),
            loaded_modules: HashSet::new(),
            blockchain_context: BlockchainContext {
                chain_id: 1,
                chain_name: "ethereum".to_string(),
                sender: "0x0000000000000000000000000000000000000000".to_string(),
                msg_value: 0,
                msg_data: String::new(),
                tx_hash: "0x0".to_string(),
                tx_timestamp: 0,
            },
        }
    }

    pub fn set_blockchain_context(&mut self, context: BlockchainContext) {
        self.blockchain_context = context;
    }

    pub fn run(&mut self, program: Vec<Stmt>) -> Result<(), String> {
        for stmt in program {
            match stmt {
                Stmt::Function(func) => {
                    self.functions.insert(func.name.clone(), func);
                }
                Stmt::Contract(contract) => {
                    // Initialize contract state storage
                    let mut state = HashMap::new();
                    for var in &contract.state {
                        state.insert(var.clone(), Value::Null);
                    }
                    self.contract_state.insert(contract.name.clone(), state);
                    
                    // Store contract definition
                    self.contracts.insert(contract.name.clone(), contract);
                }
                Stmt::Import(module) => {
                    self.load_module(&module)?;
                }
                _ => {}
            }
        }

        if let Some(main) = self.functions.get("main").cloned() {
            self.call_function(main, vec![])?;
            Ok(())
        } else {
            Err("Error: main function not found".to_string())
        }
    }

    fn execute(&mut self, stmt: Stmt) -> ExecResult {
        match stmt {
            Stmt::Let { name, value } => {
                let val = self.eval_expr(value)?;
                self.variables.insert(name, val);
                Ok(Control::Value(Value::Null))
            }
            Stmt::Expression(expr) => {
                let _ = self.eval_expr(expr)?;
                Ok(Control::Value(Value::Null))
            }
            Stmt::If { condition, then_branch, else_branch } => {
                let cond = self.eval_expr(condition)?;

                match cond {
                    Value::Bool(true) => {
                        for stmt in then_branch {
                            match self.execute(stmt)? {
                                Control::Return(v) => return Ok(Control::Return(v)),
                                Control::Value(_) => {}
                            }
                        }
                    }
                    Value::Bool(false) => {
                        for stmt in else_branch {
                            match self.execute(stmt)? {
                                Control::Return(v) => return Ok(Control::Return(v)),
                                Control::Value(_) => {}
                            }
                        }
                    }
                    _ => return Err("Error: condition must be boolean".to_string()),
                }

                Ok(Control::Value(Value::Null))
            }
            Stmt::While { condition, body } => {
                loop {
                    let cond = self.eval_expr(condition.clone())?;

                    match cond {
                        Value::Bool(true) => {
                            for stmt in body.clone() {
                                match self.execute(stmt)? {
                                    Control::Return(v) => return Ok(Control::Return(v)),
                                    Control::Value(_) => {}
                                }
                            }
                        }
                        Value::Bool(false) => break,
                        _ => return Err("Error: while condition must be boolean".to_string()),
                    }
                }

                Ok(Control::Value(Value::Null))
            }
            Stmt::Assign { name, value } => {
                let val = self.eval_expr(value)?;
                if self.variables.contains_key(&name) {
                    self.variables.insert(name.clone(), val);
                } else {
                    return Err(format!("Error: variable '{}' not defined", name));
                }
                Ok(Control::Value(Value::Null))
            }
            Stmt::Return(expr) => {
                let val = self.eval_expr(expr)?;
                Ok(Control::Return(val))
            }
            Stmt::Function(_) => Ok(Control::Value(Value::Null)),
            Stmt::Contract(_) => Ok(Control::Value(Value::Null)),
            Stmt::Import(module) => {
                self.load_module(&module)?;
                Ok(Control::Value(Value::Null))
            }
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> EvalResult {
        match expr {
            Expr::StringLiteral(v) => Ok(Value::String(v)),
            Expr::NumberLiteral(n) => Ok(Value::Number(n)),
            Expr::BoolLiteral(b) => Ok(Value::Bool(b)),
            Expr::ArrayLiteral(items) => {
                let mut values = Vec::new();
                for item in items {
                    values.push(self.eval_expr(item)?);
                }
                Ok(Value::Array(values))
            }
            Expr::Identifier(name) => match self.variables.get(&name) {
                Some(v) => Ok(v.clone()),
                None => Err(format!("Error: variable '{}' not defined", name)),
            },
            Expr::Property { object, property } => {
                self.resolve_property(&object, &property)
            }
            Expr::AICall { method, args } => {
                self.call_ai(&method, args)
            }
            Expr::Binary { left, operator, right } => {
                let l = self.eval_expr(*left)?;
                let r = self.eval_expr(*right)?;

                match (l, r, operator.as_str()) {
                    (Value::Number(a), Value::Number(b), "+") => Ok(Value::Number(a + b)),
                    (Value::Number(a), Value::Number(b), "<") => Ok(Value::Bool(a < b)),
                    (Value::Number(a), Value::Number(b), ">") => Ok(Value::Bool(a > b)),
                    (Value::Number(_), Value::Number(0), "/") => Err("Error: division by zero".to_string()),
                    (Value::Number(a), Value::Number(b), "/") => Ok(Value::Number(a / b)),
                    _ => Err("Error: invalid operation".to_string()),
                }
            }
            Expr::Call { name, args } => self.call(name, args),
        }
    }

    fn call(&mut self, name: String, args: Vec<Expr>) -> EvalResult {
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.eval_expr(arg)?);
        }

        // Handle built-in contract functions
        if name == "panic" {
            let msg = match arg_values.get(0) {
                Some(Value::String(s)) => s.clone(),
                _ => "Contract panic".to_string(),
            };
            return Err(format!("Panic: {}", msg));
        }

        if name == "transfer" {
            // Built-in transfer function (no-op for now, would interact with blockchain)
            return Ok(Value::Null);
        }

        if name == "emit" {
            // Built-in emit for contract events (no-op for now)
            return Ok(Value::Null);
        }

        if let Ok(val) = self.stdlib(&name, &arg_values) {
            return Ok(val);
        }

        if name == "print" {
            match arg_values.get(0) {
                Some(Value::String(s)) => println!("{}", s),
                Some(Value::Number(n)) => println!("{}", n),
                Some(Value::Bool(b)) => println!("{}", b),
                Some(Value::Array(arr)) => {
                    let rendered: Vec<String> = arr.iter().map(|v| self.render_value(v)).collect();
                    println!("[{}]", rendered.join(","));
                }
                Some(Value::Address(addr)) => println!("{}", addr),
                Some(Value::U256(n)) => println!("{}", n),
                Some(Value::AIResult { label, score }) => println!("{}: {:.2}", label, score),
                Some(Value::Null) | None => println!("null"),
            }
            return Ok(Value::Null);
        }

        let func = match self.functions.get(&name) {
            Some(f) => f.clone(),
            None => return Err(format!("Error: function '{}' not defined", name)),
        };

        self.call_function(func, arg_values)
    }

    fn call_function(&mut self, func: Function, args: Vec<Value>) -> EvalResult {
        let mut new_scope = HashMap::new();
        for (i, param) in func.params.iter().enumerate() {
            let arg_val = args.get(i).cloned().ok_or_else(|| "Error: argument missing".to_string())?;
            new_scope.insert(param.clone(), arg_val);
        }

        let old_scope = self.variables.clone();
        self.variables = new_scope;

        let mut ret = Value::Null;
        for stmt in func.body.clone() {
            match self.execute(stmt)? {
                Control::Return(v) => {
                    ret = v;
                    break;
                }
                Control::Value(_) => {}
            }
        }

        self.variables = old_scope;
        Ok(ret)
    }

    fn render_value(&self, v: &Value) -> String {
        match v {
            Value::String(s) => format!("\"{}\"", s),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Array(arr) => {
                let rendered: Vec<String> = arr.iter().map(|x| self.render_value(x)).collect();
                format!("[{}]", rendered.join(","))
            }
            Value::Address(addr) => addr.clone(),
            Value::U256(n) => n.to_string(),
            Value::AIResult { label, score } => format!("{}: {:.2}", label, score),
            Value::Null => "null".to_string(),
        }
    }

    fn resolve_property(&self, object: &str, property: &str) -> EvalResult {
        match (object, property) {
            // chain properties
            ("chain", "id") => Ok(Value::Number(self.blockchain_context.chain_id)),
            ("chain", "name") => Ok(Value::String(self.blockchain_context.chain_name.clone())),
            
            // msg properties
            ("msg", "sender") => Ok(Value::Address(self.blockchain_context.sender.clone())),
            ("msg", "value") => Ok(Value::U256(self.blockchain_context.msg_value)),
            ("msg", "data") => Ok(Value::String(self.blockchain_context.msg_data.clone())),
            
            // tx properties
            ("tx", "hash") => Ok(Value::String(self.blockchain_context.tx_hash.clone())),
            ("tx", "value") => Ok(Value::U256(self.blockchain_context.msg_value)),
            ("tx", "timestamp") => Ok(Value::Number(self.blockchain_context.tx_timestamp)),
            
            // AI result properties
            _ => Err(format!("Error: unknown property '{}.{}'", object, property)),
        }
    }

    fn call_ai(&mut self, method: &str, args: Vec<Expr>) -> EvalResult {
        let ai_runtime = LocalAIRuntime;
        
        match method {
            "model" => {
                if args.is_empty() {
                    return Err("ai.model() requires at least one argument".to_string());
                }
                let model_name_expr = &args[0];
                let model_name = match self.eval_expr(model_name_expr.clone())? {
                    Value::String(s) => s,
                    _ => return Err("ai.model() requires a string argument".to_string()),
                };
                
                let _model = ai_runtime.model(&model_name)?;
                // Store model name as string for now (simplified)
                Ok(Value::String(model_name))
            }
            "infer" => {
                if args.len() < 2 {
                    return Err("ai.infer() requires two arguments: model and input".to_string());
                }
                
                let _model_val = self.eval_expr(args[0].clone())?;
                let input_val = self.eval_expr(args[1].clone())?;
                
                let input_str = match input_val {
                    Value::String(s) => s,
                    _ => return Err("ai.infer() input must be a string".to_string()),
                };
                
                let model = ai_runtime.model("sentiment")?;
                ai_runtime.infer(&model, &input_str)
            }
            "embed" => {
                if args.is_empty() {
                    return Err("ai.embed() requires a text argument".to_string());
                }
                
                let text_val = self.eval_expr(args[0].clone())?;
                let text_str = match text_val {
                    Value::String(s) => s,
                    _ => return Err("ai.embed() requires a string argument".to_string()),
                };
                
                let embeddings = ai_runtime.embed(&text_str)?;
                Ok(Value::Array(
                    embeddings
                        .iter()
                        .map(|&f| Value::Number((f * 100.0) as i64))
                        .collect(),
                ))
            }
            "tokenize" => {
                if args.is_empty() {
                    return Err("ai.tokenize() requires a text argument".to_string());
                }
                
                let text_val = self.eval_expr(args[0].clone())?;
                let text_str = match text_val {
                    Value::String(s) => s,
                    _ => return Err("ai.tokenize() requires a string argument".to_string()),
                };
                
                let tokens = ai_runtime.tokenize(&text_str)?;
                Ok(Value::Array(
                    tokens.into_iter().map(Value::String).collect(),
                ))
            }
            _ => Err(format!("Unknown AI method: ai.{}", method)),
        }
    }

    fn load_module(&mut self, name: &str) -> Result<(), String> {
        if self.loaded_modules.contains(name) {
            return Ok(());
        }

        self.loaded_modules.insert(name.to_string());

        // First, try to load from installed packages
        let source = if let Ok(pm) = PackageManager::new() {
            if let Some(package_path) = pm.resolve_import(name) {
                std::fs::read_to_string(&package_path)
                    .map_err(|_| format!("Error: Cannot read package module '{}'", name))?
            } else {
                // Fall back to local file
                let filename = format!("{}.ax", name);
                std::fs::read_to_string(&filename)
                    .map_err(|_| format!("Error: Cannot find module '{}' (tried package and local file)", name))?
            }
        } else {
            // Package manager not available, try local file
            let filename = format!("{}.ax", name);
            std::fs::read_to_string(&filename)
                .map_err(|_| format!("Error: Cannot find module '{}'", name))?
        };

        let mut lexer = Lexer::new(&source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

        for stmt in ast {
            match stmt {
                Stmt::Function(func) => {
                    self.functions.insert(func.name.clone(), func);
                }
                Stmt::Import(module) => {
                    self.load_module(&module)?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn stdlib(&mut self, name: &str, args: &[Value]) -> Result<Value, String> {
        match name {
            "len" => {
                match args.get(0) {
                    Some(Value::Array(arr)) => Ok(Value::Number(arr.len() as i64)),
                    Some(Value::String(s)) => Ok(Value::Number(s.len() as i64)),
                    _ => Err("Error: len() expects array or string".to_string()),
                }
            }
            "type" => {
                let t = match args.get(0) {
                    Some(Value::Number(_)) => "number",
                    Some(Value::String(_)) => "string",
                    Some(Value::Bool(_)) => "bool",
                    Some(Value::Array(_)) => "array",
                    Some(Value::Address(_)) => "address",
                    Some(Value::U256(_)) => "u256",
                    Some(Value::AIResult { .. }) => "ai_result",
                    Some(Value::Null) => "null",
                    None => "unknown",
                };
                Ok(Value::String(t.to_string()))
            }
            "input" => {
                use std::io::{self, Write};
                if let Some(Value::String(prompt)) = args.get(0) {
                    print!("{}", prompt);
                }
                io::stdout().flush().map_err(|e| e.to_string())?;
                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer).map_err(|e| e.to_string())?;
                Ok(Value::String(buffer.trim().to_string()))
            }
            "range" => {
                if let (Some(Value::Number(start)), Some(Value::Number(end))) = (args.get(0), args.get(1)) {
                    let v: Vec<Value> = (*start..*end).map(Value::Number).collect();
                    Ok(Value::Array(v))
                } else {
                    Err("Error: range() expects two numbers".to_string())
                }
            }
            _ => Err(format!("Error: unknown stdlib function '{}'", name)),
        }
    }
}
