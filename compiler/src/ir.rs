/// ASTRIXA Intermediate Representation (IR)
/// 
/// The IR is a simplified, linear representation of the program that is:
/// - Easy to analyze
/// - Easy to optimize
/// - Easy to target multiple backends (WASM, native, bytecode)
/// 
/// Design principles:
/// - Stack-based (easy to translate to bytecode/WASM)
/// - Explicit operations (no hidden conversions)
/// - Type-erased (types are checked before lowering)

#[derive(Debug, Clone, PartialEq)]
pub enum IRInstr {
    // Constants
    LoadConstInt(i64),
    LoadConstFloat(f64),
    LoadConstBool(bool),
    LoadConstString(String),
    
    // Variables
    LoadVar(String),      // Load variable by name (legacy)
    StoreVar(String),     // Store variable by name (legacy)
    LoadLocal(u32),       // Load from local slot (new for Step 42)
    StoreLocal(u32),      // Store to local slot (new for Step 42)
    
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    
    // Comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    
    // Logical
    And,
    Or,
    Not,
    
    // Control flow
    Jump(usize),           // Jump to instruction index
    JumpIfFalse(usize),    // Conditional jump
    Call(String, usize),   // Function name, arg count
    CallStd(String),       // Call standard library function (runtime-provided)
    CallAI(String),        // STEP 52: Call AI function (runtime-provided AI)
    CallWeb3(String),      // STEP 53: Call Web3 function (runtime-provided Web3)
    CallFS(String),        // STEP 54: Call file system function (runtime-provided FS)
    Return,
    Panic,                 // STEP 48: Panic - abort execution with error
    
    // Stack manipulation
    Pop,
    Dup,                   // Duplicate top of stack
    
    // Special
    Nop,                   // No operation
}

#[derive(Debug, Clone)]
pub struct IRFunction {
    pub name: String,
    pub param_count: usize,  // STEP 46: Number of parameters
    pub instructions: Vec<IRInstr>,
    pub local_count: usize,  // Number of local variables
}

impl IRFunction {
    pub fn new(name: String) -> Self {
        Self {
            name,
            param_count: 0,  // STEP 46: Initialize to 0
            instructions: Vec::new(),
            local_count: 0,
        }
    }
    
    pub fn add_instruction(&mut self, instr: IRInstr) {
        self.instructions.push(instr);
    }
}

#[derive(Debug, Clone)]
pub struct IRModule {
    pub functions: Vec<IRFunction>,
}

impl IRModule {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }
    
    pub fn add_function(&mut self, function: IRFunction) {
        self.functions.push(function);
    }
    
    pub fn find_function(&self, name: &str) -> Option<&IRFunction> {
        self.functions.iter().find(|f| f.name == name)
    }
}
