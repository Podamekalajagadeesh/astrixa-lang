// ASTRIXA Bytecode Instructions

#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    LoadConst,      // Push constant to stack (operand: value)
    LoadVar,        // Push variable to stack (operand: var name)
    StoreVar,       // Pop from stack, store to variable (operand: var name)
    Add,            // Pop 2, add, push result
    Sub,            // Pop 2, subtract, push result
    Mul,            // Pop 2, multiply, push result
    Div,            // Pop 2, divide, push result
    Mod,            // Pop 2, modulo, push result
    Equal,          // Pop 2, check equality, push bool
    NotEqual,       // Pop 2, check inequality, push bool
    Greater,        // Pop 2, compare, push bool
    Less,           // Pop 2, compare, push bool
    GreaterEqual,   // Pop 2, compare, push bool
    LessEqual,      // Pop 2, compare, push bool
    JumpIfFalse,    // Pop stack, if false jump (operand: instruction index)
    Jump,           // Unconditional jump (operand: instruction index)
    Call,           // Call function (operand: function name)
    Return,         // Return from function
    Print,          // Print top of stack
    Pop,            // Discard top of stack
    Array,          // Create array from top N items (operand: count)
    Index,          // Index into array/string (pop 2: index, array)
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: OpCode,
    pub operand: Option<String>,
}

impl Instruction {
    pub fn new(opcode: OpCode, operand: Option<String>) -> Self {
        Instruction { opcode, operand }
    }

    pub fn const_instr(value: String) -> Self {
        Instruction::new(OpCode::LoadConst, Some(value))
    }

    pub fn var_instr(name: String) -> Self {
        Instruction::new(OpCode::LoadVar, Some(name))
    }

    pub fn store_var(name: String) -> Self {
        Instruction::new(OpCode::StoreVar, Some(name))
    }

    pub fn jump_instr(target: usize) -> Self {
        Instruction::new(OpCode::Jump, Some(target.to_string()))
    }

    pub fn jump_if_false(target: usize) -> Self {
        Instruction::new(OpCode::JumpIfFalse, Some(target.to_string()))
    }

    pub fn call_instr(name: String) -> Self {
        Instruction::new(OpCode::Call, Some(name))
    }

    pub fn array_instr(count: usize) -> Self {
        Instruction::new(OpCode::Array, Some(count.to_string()))
    }
}
