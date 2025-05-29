use std::{cell::RefCell, path::PathBuf, process::exit, rc::Rc};

use threadpool::ThreadPool;

use crate::{
    diagnostics::DiagnosticBag,
    drivers::fs_manager::FilesystemManager,
    execution::PlanExecutor,
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
    pub config: Config,
    pub args: Args,
    pub plan: Plan,
    pub thread_pool: ThreadPool,
}

#[derive(Default)]
pub struct Core {
    ctx: Context,
    diagnostics: DiagnosticsCell,
    fs_m: FilesystemManagerCell,
}

impl Core {
    pub fn parse_args(&mut self, args: Vec<String>) {
        let mut parser = ArgParser::new(args, &mut self.ctx.args);

        if let Err(err) = parser.try_parse() {
            self.diagnostics.borrow_mut().report_error(err);
            Logger::info(SHORT_HELP);
        }
    }

    pub fn parse_config(&mut self) {
        let cfg_path = PathBuf::from(CONFIG_FILE_PATH);
        if !cfg_path.exists() {
            Logger::error("Cum.toml file is missing, try 'cum init' first");
            exit(1);
        }

        let mut parser = ConfigParser::new(cfg_path, &mut self.ctx.config);

        if let Err(err) = parser.make_default() {
            self.diagnostics.borrow_mut().report_error(err);
        }
        if let Err(err) = parser.try_incremental_parse() {
            self.diagnostics.borrow_mut().report_error(err);
        }
    }

    pub fn make_plan(&mut self) {
        let mut planner = Planner::new(&mut self.ctx, self.fs_m.clone());

        Logger::info("Analyzing dependencies...");

        if let Err(err) = planner.try_make_plan() {
            self.diagnostics.borrow_mut().report_error(err);
        }
    }

    pub fn execute_plan(&mut self) {
        let executor = PlanExecutor::new(&self.ctx, self.fs_m.clone(), self.diagnostics.clone());
        executor.execute_and_report();
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
        if PrintHelp.is_satisfied_by(&self.ctx.args) {
            Logger::info(HELP_MSG);
            exit(0);
        } else if PrintVersion.is_satisfied_by(&self.ctx.args) {
            Logger::info(VERSION_MSG);
            exit(0);
        }
    }

    pub fn ctx(&self) -> &Context {
        &self.ctx
    }
}
