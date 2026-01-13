use std::collections::HashMap;

use crate::ir::{IRFunction, IRInstr, IRModule};

/// Inline small, branch-free functions directly at call sites.
pub fn inline_small_functions(module: &mut IRModule) {
    let candidates = collect_candidates(module);

    for function in module.functions.iter_mut() {
        inline_in_function(function, &candidates);
    }
}

#[derive(Clone)]
struct InlineCandidate {
    instrs: Vec<IRInstr>,
    param_count: usize,
    local_count: usize,
}

fn collect_candidates(module: &IRModule) -> HashMap<String, InlineCandidate> {
    let mut candidates = HashMap::new();

    for func in &module.functions {
        if is_inline_candidate(func) {
            candidates.insert(
                func.name.clone(),
                InlineCandidate {
                    instrs: func.instructions.clone(),
                    param_count: func.param_count,
                    local_count: func.local_count,
                },
            );
        }
    }

    candidates
}

fn is_inline_candidate(func: &IRFunction) -> bool {
    if func.instructions.len() > 5 {
        return false;
    }

    if func.param_count > 5 {
        return false;
    }

    let mut return_count = 0;
    for instr in &func.instructions {
        match instr {
            IRInstr::Jump(_) | IRInstr::JumpIfFalse(_) => return false,
            IRInstr::Call(_, _) | IRInstr::CallStd(_) | IRInstr::CallAI(_) => return false,
            IRInstr::Return => return_count += 1,
            _ => {}
        }
    }

    return_count == 1 && func.instructions.last().map(|i| matches!(i, IRInstr::Return)).unwrap_or(false)
}

fn inline_in_function(function: &mut IRFunction, candidates: &HashMap<String, InlineCandidate>) {
    let mut new_instrs = Vec::new();
    let mut i = 0;

    while i < function.instructions.len() {
        match &function.instructions[i] {
            IRInstr::Call(name, arg_count) => {
                if let Some(callee) = candidates.get(name) {
                    if callee.param_count == *arg_count {
                        let base = function.local_count;
                        let added_locals = callee.local_count;
                        function.local_count += added_locals;

                        // Store arguments into the newly reserved local slots (reverse order to pop correctly)
                        for idx in (0..*arg_count).rev() {
                            new_instrs.push(IRInstr::StoreLocal((base + idx) as u32));
                        }

                        // Emit callee body with local indices shifted
                        for instr in &callee.instrs {
                            match instr {
                                IRInstr::Return => {}
                                IRInstr::LoadLocal(slot) => {
                                    new_instrs.push(IRInstr::LoadLocal(base as u32 + slot));
                                }
                                IRInstr::StoreLocal(slot) => {
                                    new_instrs.push(IRInstr::StoreLocal(base as u32 + slot));
                                }
                                other => new_instrs.push(other.clone()),
                            }
                        }

                        i += 1;
                        continue;
                    }
                }

                new_instrs.push(function.instructions[i].clone());
                i += 1;
            }
            _ => {
                new_instrs.push(function.instructions[i].clone());
                i += 1;
            }
        }
    }

    function.instructions = new_instrs;
}
