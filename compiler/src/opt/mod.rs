//! IR optimization pipeline (Step 51)
//!
//! Runs a small set of targeted passes to make IR lean before codegen:
//! - Constant folding: shrink pure expressions
//! - Dead code elimination: strip unreachable tails
//! - Basic inlining: inline tiny branch-free functions

mod const_fold;
mod dce;
mod inline;

use crate::ir::{IRInstr, IRModule};

pub use const_fold::const_fold;
pub use dce::dead_code_elim;
pub use inline::inline_small_functions;

/// Run all instruction-level passes on a single function body.
pub fn optimize(ir: &mut Vec<IRInstr>) {
    const_fold(ir);
    dead_code_elim(ir);
}

/// Run optimization passes across an entire module.
pub fn optimize_module(module: &IRModule) -> IRModule {
    let mut optimized = module.clone();

    for func in optimized.functions.iter_mut() {
        optimize(&mut func.instructions);
    }

    inline_small_functions(&mut optimized);

    // Re-run lightweight passes to clean up any new opportunities exposed by inlining.
    for func in optimized.functions.iter_mut() {
        optimize(&mut func.instructions);
    }

    optimized
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::IRFunction;

    #[test]
    fn folds_constant_addition() {
        let mut ir = vec![
            IRInstr::LoadConstInt(10),
            IRInstr::LoadConstInt(20),
            IRInstr::Add,
            IRInstr::Return,
        ];

        optimize(&mut ir);

        assert_eq!(ir.len(), 2);
        assert!(matches!(ir[0], IRInstr::LoadConstInt(30)));
    }

    #[test]
    fn truncates_dead_code_after_return() {
        let mut ir = vec![
            IRInstr::LoadConstInt(1),
            IRInstr::Return,
            IRInstr::LoadConstInt(2),
        ];

        optimize(&mut ir);

        assert_eq!(ir.len(), 2);
        assert!(matches!(ir[1], IRInstr::Return));
    }

    #[test]
    fn inlines_small_function_body() {
        let mut module = IRModule {
            functions: vec![
                IRFunction {
                    name: "add".to_string(),
                    param_count: 2,
                    instructions: vec![
                        IRInstr::LoadLocal(0),
                        IRInstr::LoadLocal(1),
                        IRInstr::Add,
                        IRInstr::Return,
                    ],
                    local_count: 2,
                },
                IRFunction {
                    name: "main".to_string(),
                    param_count: 0,
                    instructions: vec![
                        IRInstr::LoadConstInt(2),
                        IRInstr::LoadConstInt(3),
                        IRInstr::Call("add".to_string(), 2),
                        IRInstr::Return,
                    ],
                    local_count: 0,
                },
            ],
        };

        let optimized = optimize_module(&module);
        let main = optimized.functions.iter().find(|f| f.name == "main").unwrap();

        assert!(main.instructions.iter().all(|i| !matches!(i, IRInstr::Call(_, _))));
        assert_eq!(main.local_count, 2);
        assert!(matches!(main.instructions.last(), Some(IRInstr::Return)));
    }
}
