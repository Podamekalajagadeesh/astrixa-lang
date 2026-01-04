// ASTRIXA Virtual Machine: Executes Bytecode

use std::collections::HashMap;
use crate::bytecode::{OpCode, Instruction};
use crate::interpreter::{Value, BlockchainContext}; // Import both Value and BlockchainContext
use crate::gas::{gas_cost, GasContext};

pub struct VM {
    stack: Vec<Value>,
    vars: HashMap<String, Value>,
    ip: usize, // Instruction pointer
    call_stack: Vec<usize>, // For function returns
    blockchain_context: BlockchainContext,
    gas_context: GasContext,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Vec::new(),
            vars: HashMap::new(),
            ip: 0,
            call_stack: Vec::new(),
            blockchain_context: BlockchainContext {
                chain_id: 1,
                chain_name: "ethereum".to_string(),
                sender: "0x0000000000000000000000000000000000000000".to_string(),
                msg_value: 0,
                msg_data: String::new(),
                tx_hash: "0x0".to_string(),
                tx_timestamp: 0,
            },
            gas_context: GasContext::new(1_000_000, 1), // Default: 1M gas at 1 wei/gas
        }
    }

    pub fn with_gas(mut self, gas_limit: u64, gas_price: u64) -> Self {
        self.gas_context = GasContext::new(gas_limit, gas_price);
        self
    }

    pub fn set_blockchain_context(&mut self, context: BlockchainContext) {
        self.blockchain_context = context;
    }

    pub fn gas_used(&self) -> u64 {
        self.gas_context.gas_used
    }

    pub fn gas_remaining(&self) -> u64 {
        self.gas_context.remaining()
    }

    pub fn run(&mut self, instructions: Vec<Instruction>) -> Result<Value, String> {
        while self.ip < instructions.len() {
            let instr = &instructions[self.ip];

            // Deduct gas before executing instruction
            let cost = gas_cost(&instr.opcode);
            self.gas_context.gas_used += cost;

            // Check if we've exceeded gas limit
            if self.gas_context.is_out_of_gas() {
                return Err(format!(
                    "Out of gas: used {} gas, limit was {} gas",
                    self.gas_context.gas_used, self.gas_context.gas_limit
                ));
            }

            match &instr.opcode {
                OpCode::LoadConst => {
                    let value = self.parse_constant(instr.operand.clone().unwrap())?;
                    self.stack.push(value);
                }
                OpCode::LoadVar => {
                    let name = instr.operand.clone().unwrap();
                    
                    // Check if it's a property access (e.g., "msg.sender")
                    let value = if name.contains('.') {
                        let parts: Vec<&str> = name.split('.').collect();
                        if parts.len() == 2 {
                            self.resolve_property(parts[0], parts[1])?
                        } else {
                            return Err(format!("Invalid property access: {}", name));
                        }
                    } else {
                        self.vars.get(&name)
                            .cloned()
                            .ok_or(format!("Undefined variable: {}", name))?
                    };
                    self.stack.push(value);
                }
                OpCode::StoreVar => {
                    let name = instr.operand.clone().unwrap();
                    let value = self.stack.last()
                        .cloned()
                        .ok_or("Stack underflow")?;
                    self.vars.insert(name, value);
                }
                OpCode::Pop => {
                    self.stack.pop();
                }
                OpCode::Add => {
                    self.binary_op(|a, b| match (a, b) {
                        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x + y)),
                        (Value::String(x), Value::String(y)) => Ok(Value::String(x + &y)),
                        _ => Err("Type error in Add".to_string()),
                    })?;
                }
                OpCode::Sub => {
                    self.binary_op(|a, b| match (a, b) {
                        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x - y)),
                        _ => Err("Type error in Sub".to_string()),
                    })?;
                }
                OpCode::Mul => {
                    self.binary_op(|a, b| match (a, b) {
                        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x * y)),
                        _ => Err("Type error in Mul".to_string()),
                    })?;
                }
                OpCode::Div => {
                    self.binary_op(|a, b| match (a, b) {
                        (Value::Number(x), Value::Number(0)) => Err("Division by zero".to_string()),
                        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x / y)),
                        _ => Err("Type error in Div".to_string()),
                    })?;
                }
                OpCode::Greater => {
                    self.binary_op(|a, b| match (a, b) {
                        (Value::Number(x), Value::Number(y)) => Ok(Value::Bool(x > y)),
                        _ => Err("Type error in Greater".to_string()),
                    })?;
                }
                OpCode::Less => {
                    self.binary_op(|a, b| match (a, b) {
                        (Value::Number(x), Value::Number(y)) => Ok(Value::Bool(x < y)),
                        _ => Err("Type error in Less".to_string()),
                    })?;
                }
                OpCode::Jump => {
                    let target = instr.operand.clone().unwrap().parse::<usize>()
                        .map_err(|_| "Invalid jump target".to_string())?;
                    self.ip = target;
                    continue;
                }
                OpCode::JumpIfFalse => {
                    let cond = self.stack.pop().ok_or("Stack underflow")?;
                    let is_false = match cond {
                        Value::Bool(false) => true,
                        Value::Null => true,
                        _ => false,
                    };

                    if is_false {
                        let target = instr.operand.clone().unwrap().parse::<usize>()
                            .map_err(|_| "Invalid jump target".to_string())?;
                        self.ip = target;
                        continue;
                    }
                }
                OpCode::Call => {
                    let name = instr.operand.clone().unwrap();
                    self.call_stdlib(&name)?;
                }
                OpCode::Return => {
                    let value = self.stack.pop().unwrap_or(Value::Null);
                    return Ok(value);
                }
                OpCode::Print => {
                    if let Some(value) = self.stack.pop() {
                        match value {
                            Value::Number(n) => println!("{}", n),
                            Value::String(s) => println!("{}", s),
                            Value::Bool(b) => println!("{}", b),
                            Value::Array(arr) => {
                                let rendered: Vec<String> = arr.iter().map(|v| self.render_value(v)).collect();
                                println!("[{}]", rendered.join(","));
                            }
                            Value::Null => println!("null"),
                        }
                    }
                }
                OpCode::Array => {
                    let count = instr.operand.clone().unwrap().parse::<usize>()
                        .map_err(|_| "Invalid array size".to_string())?;
                    let mut arr = Vec::new();
                    for _ in 0..count {
                        if let Some(val) = self.stack.pop() {
                            arr.insert(0, val);
                        }
                    }
                    self.stack.push(Value::Array(arr));
                }
                OpCode::Index => {
                    let idx = self.stack.pop().ok_or("Stack underflow")?;
                    let obj = self.stack.pop().ok_or("Stack underflow")?;

                    match (obj, idx) {
                        (Value::Array(arr), Value::Number(i)) => {
                            let val = arr.get(i as usize)
                                .cloned()
                                .ok_or("Index out of bounds".to_string())?;
                            self.stack.push(val);
                        }
                        (Value::String(s), Value::Number(i)) => {
                            let ch = s.chars().nth(i as usize)
                                .ok_or("Index out of bounds".to_string())?;
                            self.stack.push(Value::String(ch.to_string()));
                        }
                        _ => return Err("Invalid indexing".to_string()),
                    }
                }
            }

            self.ip += 1;
        }

        Ok(Value::Null)
    }

    fn binary_op<F>(&mut self, op: F) -> Result<(), String>
    where
        F: Fn(Value, Value) -> Result<Value, String>,
    {
        let b = self.stack.pop().ok_or("Stack underflow")?;
        let a = self.stack.pop().ok_or("Stack underflow")?;
        let result = op(a, b)?;
        self.stack.push(result);
        Ok(())
    }

    fn parse_constant(&self, s: String) -> Result<Value, String> {
        if s == "null" {
            Ok(Value::Null)
        } else if s == "true" {
            Ok(Value::Bool(true))
        } else if s == "false" {
            Ok(Value::Bool(false))
        } else if s.starts_with('"') && s.ends_with('"') {
            Ok(Value::String(s[1..s.len()-1].to_string()))
        } else if let Ok(n) = s.parse::<i64>() {
            Ok(Value::Number(n))
        } else {
            Err(format!("Unknown constant: {}", s))
        }
    }

    fn resolve_property(&self, object: &str, property: &str) -> Result<Value, String> {
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
            
            _ => Err(format!("Unknown property: {}.{}", object, property)),
        }
    }

    fn call_stdlib(&mut self, name: &str) -> Result<(), String> {
        // Handle AI calls
        if name.starts_with("ai.") {
            return self.call_ai(name);
        }
        
        match name {
            "print" => {
                if let Some(value) = self.stack.pop() {
                    match value {
                        Value::Number(n) => println!("{}", n),
                        Value::String(s) => println!("{}", s),
                        Value::Bool(b) => println!("{}", b),
                        Value::Array(arr) => {
                            let rendered: Vec<String> = arr.iter().map(|v| self.render_value(v)).collect();
                            println!("[{}]", rendered.join(","));
                        }
                        Value::Address(addr) => println!("{}", addr),
                        Value::U256(n) => println!("{}", n),
                        Value::AIResult { label, score } => println!("{}: {:.2}", label, score),
                        Value::Null => println!("null"),
                    }
                    self.stack.push(Value::Null);
                }
                Ok(())
            }
            "len" => {
                let val = self.stack.pop().ok_or("Stack underflow")?;
                let len = match val {
                    Value::Array(ref arr) => arr.len() as i64,
                    Value::String(ref s) => s.len() as i64,
                    _ => return Err("len() expects array or string".to_string()),
                };
                self.stack.push(Value::Number(len));
                Ok(())
            }
            "type" => {
                let val = self.stack.pop().ok_or("Stack underflow")?;
                let type_str = match val {
                    Value::Number(_) => "number",
                    Value::String(_) => "string",
                    Value::Bool(_) => "bool",
                    Value::Array(_) => "array",
                    Value::Address(_) => "address",
                    Value::U256(_) => "u256",
                    Value::AIResult { .. } => "ai_result",
                    Value::Null => "null",
                };
                self.stack.push(Value::String(type_str.to_string()));
                Ok(())
            }
            "range" => {
                let end = self.stack.pop().ok_or("Stack underflow")?;
                let start = self.stack.pop().ok_or("Stack underflow")?;

                if let (Value::Number(s), Value::Number(e)) = (start, end) {
                    let arr: Vec<Value> = (s..e).map(Value::Number).collect();
                    self.stack.push(Value::Array(arr));
                    Ok(())
                } else {
                    Err("range() expects two numbers".to_string())
                }
            }
            _ => Err(format!("Unknown function: {}", name)),
        }
    }

    fn call_ai(&mut self, name: &str) -> Result<(), String> {
        use crate::ai_runtime::{AIRuntime, LocalAIRuntime};
        
        let method = name.strip_prefix("ai.").unwrap_or("");
        let ai_runtime = LocalAIRuntime;

        match method {
            "infer" => {
                // Pop input and model from stack (model on top)
                let input = self.stack.pop().ok_or("Stack underflow")?;
                let _model = self.stack.pop().ok_or("Stack underflow")?;

                let input_str = match input {
                    Value::String(s) => s,
                    _ => return Err("ai.infer() requires string input".to_string()),
                };

                // For now, use a default sentiment model
                let model = ai_runtime.model("sentiment")?;
                let result = ai_runtime.infer(&model, &input_str)?;
                self.stack.push(result);
                Ok(())
            }
            "embed" => {
                let text = self.stack.pop().ok_or("Stack underflow")?;

                let text_str = match text {
                    Value::String(s) => s,
                    _ => return Err("ai.embed() requires string input".to_string()),
                };

                let embeddings = ai_runtime.embed(&text_str)?;
                let values: Vec<Value> = embeddings
                    .iter()
                    .map(|&f| Value::Number((f * 100.0) as i64))
                    .collect();
                self.stack.push(Value::Array(values));
                Ok(())
            }
            "tokenize" => {
                let text = self.stack.pop().ok_or("Stack underflow")?;

                let text_str = match text {
                    Value::String(s) => s,
                    _ => return Err("ai.tokenize() requires string input".to_string()),
                };

                let tokens = ai_runtime.tokenize(&text_str)?;
                let values: Vec<Value> = tokens.into_iter().map(Value::String).collect();
                self.stack.push(Value::Array(values));
                Ok(())
            }
            "model" => {
                let model_name = self.stack.pop().ok_or("Stack underflow")?;

                let name_str = match model_name {
                    Value::String(s) => s,
                    _ => return Err("ai.model() requires string argument".to_string()),
                };

                let _model = ai_runtime.model(&name_str)?;
                self.stack.push(Value::String(name_str));
                Ok(())
            }
            _ => Err(format!("Unknown AI method: ai.{}", method)),
        }
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
}
