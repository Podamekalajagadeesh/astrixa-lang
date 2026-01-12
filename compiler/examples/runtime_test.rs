/// Test the runtime with a simple WASM program
/// This creates IR directly and tests the WASM generation and runtime

use astrixa::ir::{IRFunction, IRInstr, IRModule};
use astrixa::codegen::wasm;

fn main() {
    println!("🧪 ASTRIXA Runtime Test\n");
    
    // Create a simple IR program: println(42)
    let mut module = IRModule::new();
    let mut func = IRFunction::new("main".to_string());
    
    // Load constant 42
    func.add_instruction(IRInstr::LoadConstInt(42));
    
    // Call stdlib function println
    func.add_instruction(IRInstr::CallStd("println".to_string()));
    
    // Return 0
    func.add_instruction(IRInstr::LoadConstInt(0));
    func.add_instruction(IRInstr::Return);
    
    module.add_function(func);
    
    // Generate WASM
    let wasm_code = wasm::generate_wasm_module(&module);
    
    println!("📝 Generated WASM (WAT format):\n");
    println!("{}", wasm_code);
    
    // Write to file
    std::fs::write("../runtime/test_output.wat", &wasm_code)
        .expect("Failed to write WAT file");
    
    println!("\n✅ WASM generated successfully!");
    println!("📁 Saved to: runtime/test_output.wat");
    println!("\n🚀 Run with:");
    println!("   cd runtime && node run.js test_output.wat");
}
