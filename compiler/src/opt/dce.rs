use crate::ir::IRInstr;

/// Truncate instructions that are unreachable after an early exit.
pub fn dead_code_elim(ir: &mut Vec<IRInstr>) {
    if let Some(pos) = ir
        .iter()
        .position(|i| matches!(i, IRInstr::Return | IRInstr::Panic))
    {
        ir.truncate(pos + 1);
    }
}
