/// Pretty-print compilation errors with helpful formatting
use crate::error::CompileError;

pub fn display_error(err: CompileError) {
    eprintln!("Error: {}", err.message);
    eprintln!(" → line {}, column {}", err.line, err.column);

    if let Some(help) = err.help {
        eprintln!(" Help: {}", help);
    }
}

pub fn display_errors(errors: &[CompileError]) {
    if errors.is_empty() {
        return;
    }

    for (i, err) in errors.iter().enumerate() {
        if i > 0 {
            eprintln!();
        }
        display_error(err.clone());
    }
}
