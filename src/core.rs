use std::{cell::RefCell, path::PathBuf, process::exit, rc::Rc};

use crate::{
    diagnostics::DiagnosticBag,
    drivers::fs_manager::FilesystemManager,
    logger::Logger,
    meta::{HELP_MSG, SHORT_HELP, VERSION_MSG},
    parsing::{
        arg_parser::{ArgParser, Args},
        config::{Config, CONFIG_FILE_PATH},
        config_parser::ConfigParser,
    },
    planning::{
        args_specification::{ArgsSpec, PrintHelp, PrintVersion},
        plan::Plan,
        planner::Planner,
    },
};

pub type DiagnosticsCell = Rc<RefCell<DiagnosticBag>>;
pub type FilesystemManagerCell = Rc<FilesystemManager>;

#[derive(Default)]
pub struct Context {
    pub config: Rc<RefCell<Config>>,
    pub args: Rc<RefCell<Args>>,
    pub plan: Rc<RefCell<Plan>>,
}

#[derive(Default)]
pub struct Core {
    ctx: Context,
    diagnostics: DiagnosticsCell,
    fs_m: FilesystemManagerCell,
}

impl Core {
    pub fn parse_args(&mut self, args: Vec<String>) {
        let parser = ArgParser::new(args, self.ctx.args.clone());

        if let Err(err) = parser.try_parse() {
            self.diagnostics.borrow_mut().report_error(err);
            Logger::info(SHORT_HELP);
        }
    }

    pub fn parse_config(&mut self) {
        let cfg_path = PathBuf::from(CONFIG_FILE_PATH);
        if !cfg_path.exists() {
            Logger::warning("Config file is missing, default one was loaded");
        }

        let parser = ConfigParser::new(cfg_path, self.ctx.config.clone());

        if let Err(err) = parser.make_default() {
            self.diagnostics.borrow_mut().report_error(err);
        }
        if let Err(err) = parser.try_incremental_parse() {
            self.diagnostics.borrow_mut().report_error(err);
        }
    }

    pub fn make_plan(&mut self) {
        let planner = Planner::new(
            self.ctx.config.clone(),
            self.ctx.args.clone(),
            self.ctx.plan.clone(),
            self.fs_m.clone(),
        );

        if let Err(err) = planner.try_make_plan() {
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

    /// Print information if needed and exist (if printed something).
    pub fn print_info(&self) {
        let args_bind = self.ctx.args.borrow();

        if PrintHelp.is_satisfied_by(&args_bind) {
            Logger::info(HELP_MSG);
            exit(0);
        } else if PrintVersion.is_satisfied_by(&args_bind) {
            Logger::info(VERSION_MSG);
            exit(0);
        }
    }

    pub fn ctx_ref(&self) -> &Context {
        &self.ctx
    }
}
