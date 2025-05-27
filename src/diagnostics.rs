use std::fmt::Display;

use crate::logger::Logger;

#[derive(PartialEq, Eq)]
enum DiagnosticKind {
    Info,
    Warning,
    Error,
}

struct Diagnostic {
    pub kind: DiagnosticKind,
    pub msg: String,
}

#[derive(Default)]
pub struct DiagnosticBag {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticBag {
    pub fn report_error<E: Display>(&mut self, err: E) {
        self.diagnostics.push(Diagnostic {
            kind: DiagnosticKind::Error,
            msg: err.to_string(),
        });
    }

    pub fn report_error_str(&mut self, msg: String) {
        self.diagnostics.push(Diagnostic {
            kind: DiagnosticKind::Error,
            msg,
        });
    }

    pub fn report_info(&mut self, msg: String) {
        self.diagnostics.push(Diagnostic {
            kind: DiagnosticKind::Info,
            msg,
        });
    }

    pub fn report_warning(&mut self, msg: String) {
        self.diagnostics.push(Diagnostic {
            kind: DiagnosticKind::Warning,
            msg,
        });
    }

    #[inline]
    pub fn print_all(&self) {
        for d in self.diagnostics.iter() {
            match d.kind {
                DiagnosticKind::Info => Logger::info(d.msg.clone()),
                DiagnosticKind::Warning => Logger::warning(d.msg.clone()),
                DiagnosticKind::Error => Logger::error(d.msg.clone()),
            }
        }
    }

    #[inline]
    pub fn contains_error(&self) -> bool {
        for d in self.diagnostics.iter() {
            if d.kind == DiagnosticKind::Error {
                return true;
            }
        }
        false
    }
}
