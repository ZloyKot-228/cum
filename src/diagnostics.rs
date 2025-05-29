use std::{cmp::Ordering, fmt::Display};

use crate::logger::Logger;

#[derive(PartialEq, Eq, Clone)]
enum DiagnosticKind {
    Warning,
    Error,
}

#[derive(Clone)]
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

    pub fn report_warning(&mut self, msg: String) {
        self.diagnostics.push(Diagnostic {
            kind: DiagnosticKind::Warning,
            msg,
        });
    }

    #[inline]
    pub fn print_all(&self) {
        for d in self.get_sorted() {
            match d.kind {
                DiagnosticKind::Warning => Logger::warning(&d.msg),
                DiagnosticKind::Error => Logger::error(&d.msg),
            }
        }
    }

    #[inline]
    pub fn print_all_clear(&mut self) {
        self.print_all();
        self.diagnostics.clear();
    }

    #[inline]
    pub fn get_sorted(&self) -> Vec<Diagnostic> {
        let mut sorted = self.diagnostics.clone();
        sorted.sort_by(|a, _| {
            if a.kind == DiagnosticKind::Warning {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        sorted
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
