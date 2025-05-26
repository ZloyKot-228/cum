use std::process::exit;

use crate::{
    diagnostics::{DiagnosticBag, DiagnosticBagCell},
    parsing::config::Config,
};

#[derive(Default)]
pub struct Context {
    pub config: Config,
}

pub struct Core {
    ctx: Context,
    diagnostics: DiagnosticBagCell,
}

impl Core {
    pub fn parse_config(&mut self) {
        todo!()
    }

    pub fn verify_diagnostics(&self) {
        let bind = self.diagnostics.borrow();
        if bind.contains_error() {
            bind.print_all();
            exit(1);
        }
    }

    pub fn print_diagnostics_final(self) {
        self.diagnostics.into_inner().print_all_once();
    }

    pub fn get_ctx_ref(&self) -> &Context {
        &self.ctx
    }
}

impl Default for Core {
    fn default() -> Self {
        Self {
            ctx: Context::default(),
            diagnostics: DiagnosticBag::new(),
        }
    }
}
