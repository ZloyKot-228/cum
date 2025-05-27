use std::{cell::RefCell, path::PathBuf, process::exit, rc::Rc};

use crate::{
    diagnostics::DiagnosticBag,
    logger::Logger,
    parsing::{
        arg_parser::{ArgParser, Args},
        config::{Config, CONFIG_FILE_PATH},
        config_parser::ConfigParser,
    },
};

pub const SHORT_HELP: &str = "Call 'cum --help' for help";

pub type ContextCell = Rc<RefCell<Context>>;
pub type DiagnosticsCell = Rc<RefCell<DiagnosticBag>>;

#[derive(Default)]
pub struct Context {
    pub config: Config,
    pub args: Args,
}

#[derive(Default)]
pub struct Core {
    ctx: ContextCell,
    diagnostics: DiagnosticsCell,
}

impl Core {
    pub fn parse_args(&mut self, args: Vec<String>) {
        let parser = ArgParser::new(args, self.ctx.clone());

        if let Err(err) = parser.try_parse() {
            self.diagnostics.borrow_mut().report_error(err);
            Logger::info(SHORT_HELP);
        }
    }

    pub fn parse_config(&mut self) {
        let cfg_path = PathBuf::from(CONFIG_FILE_PATH);
        if !cfg_path.exists() {
            Logger::warning("Config file is missing, default one is loaded");
        }

        let parser = ConfigParser::new(cfg_path, self.ctx.clone());

        if let Err(err) = parser.make_default() {
            self.diagnostics.borrow_mut().report_error(err);
        }
        if let Err(err) = parser.try_incremental_parse() {
            self.diagnostics.borrow_mut().report_error(err);
        }
    }

    pub fn verify_diagnostics(&self) {
        let bind = self.diagnostics.borrow();
        if bind.contains_error() {
            bind.print_all();
            exit(1);
        }
    }

    pub fn print_all_diagnostics(&self) {
        self.diagnostics.borrow().print_all();
    }

    pub fn ctx_ref(&self) -> &ContextCell {
        &self.ctx
    }
}
