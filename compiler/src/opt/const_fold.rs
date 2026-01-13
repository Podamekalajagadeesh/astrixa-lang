use crate::ir::IRInstr;

/// Fold constant arithmetic and comparison patterns inside a linear IR stream.
pub fn const_fold(ir: &mut Vec<IRInstr>) {
    let mut i = 0;
    while i + 2 < ir.len() {
        let replaced = match (&ir[i], &ir[i + 1], &ir[i + 2]) {
            (IRInstr::LoadConstInt(a), IRInstr::LoadConstInt(b), IRInstr::Add) => {
                Some(IRInstr::LoadConstInt(a + b))
            }
            (IRInstr::LoadConstInt(a), IRInstr::LoadConstInt(b), IRInstr::Sub) => {
                Some(IRInstr::LoadConstInt(a - b))
            }
            (IRInstr::LoadConstInt(a), IRInstr::LoadConstInt(b), IRInstr::Mul) => {
                Some(IRInstr::LoadConstInt(a * b))
            }
            (IRInstr::LoadConstInt(a), IRInstr::LoadConstInt(b), IRInstr::Div) if *b != 0 => {
                Some(IRInstr::LoadConstInt(a / b))
            }
            (IRInstr::LoadConstInt(a), IRInstr::LoadConstInt(b), IRInstr::Mod) if *b != 0 => {
                Some(IRInstr::LoadConstInt(a % b))
            }
            (IRInstr::LoadConstInt(a), IRInstr::LoadConstInt(b), IRInstr::Eq) => {
                Some(IRInstr::LoadConstInt(if a == b { 1 } else { 0 }))
            }
            (IRInstr::LoadConstInt(a), IRInstr::LoadConstInt(b), IRInstr::Ne) => {
                Some(IRInstr::LoadConstInt(if a != b { 1 } else { 0 }))
            }
            (IRInstr::LoadConstInt(a), IRInstr::LoadConstInt(b), IRInstr::Lt) => {
                Some(IRInstr::LoadConstInt(if a < b { 1 } else { 0 }))
            }
            (IRInstr::LoadConstInt(a), IRInstr::LoadConstInt(b), IRInstr::Le) => {
                Some(IRInstr::LoadConstInt(if a <= b { 1 } else { 0 }))
            }
            (IRInstr::LoadConstInt(a), IRInstr::LoadConstInt(b), IRInstr::Gt) => {
                Some(IRInstr::LoadConstInt(if a > b { 1 } else { 0 }))
            }
            (IRInstr::LoadConstInt(a), IRInstr::LoadConstInt(b), IRInstr::Ge) => {
                Some(IRInstr::LoadConstInt(if a >= b { 1 } else { 0 }))
            }
            _ => None,
        };

        if let Some(replacement) = replaced {
            ir.splice(i..i + 3, [replacement]);
        } else {
            i += 1;
        }
    }
}
