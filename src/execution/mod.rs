use std::{
    cell::RefCell,
    sync::mpsc::{self, Receiver, Sender},
};

use path_clean::PathClean;

use crate::{
    concurrency::timer::Timer,
    core::{Context, DiagnosticsCell, FilesystemManagerCell},
    drivers::proc_spawner::{ProcSpawnRusult, ProcSpawner},
    logger::Logger,
    meta::COMPILER,
    planning::{plan::PlanVisitor, step::Step},
};

pub struct PlanExecutor<'a> {
    ctx: &'a Context,
    fs_m: FilesystemManagerCell,
    diagnostics: DiagnosticsCell,

    compilation_timer: Timer,
    compilation_tx: RefCell<Option<Sender<ProcSpawnRusult>>>,
    compilation_rx: Receiver<ProcSpawnRusult>,
}

impl<'a> PlanExecutor<'a> {
    pub fn new(
        ctx: &'a Context,
        fs_m: FilesystemManagerCell,
        diagnostics: DiagnosticsCell,
    ) -> Self {
        let (compilation_tx, compilation_rx) = mpsc::channel();

        Self {
            ctx,
            fs_m,
            diagnostics,
            compilation_timer: Timer::default(),
            compilation_rx,
            compilation_tx: Some(compilation_tx).into(),
        }
    }

    pub fn execute_and_report(&self) {
        for step in self.ctx.plan.steps() {
            step.accept(self);
            if self.diagnostics.borrow().contains_error() {
                return;
            }
        }
    }

    /// Returns false if compilation failed.
    fn accept_compilation(&self) -> bool {
        // Wait for all compilers to finish
        self.ctx.thread_pool.join();
        // Calls destructor on tx
        *self.compilation_tx.borrow_mut() = None;
        let mut success = true;

        self.compilation_timer.stop();
        if let Some(time) = self.compilation_timer.elapsed_float() {
            Logger::info(&format!("Compilation finished at {:.2}", time));
        }

        for res in self.compilation_rx.iter() {
            match res {
                Ok(o) if o.exit_code != 0 => {
                    success = false;
                    self.diagnostics
                        .borrow_mut()
                        .report_error_str(o.errs.trim().to_string());
                }
                Ok(o) if !o.errs.is_empty() => {
                    self.diagnostics
                        .borrow_mut()
                        .report_warning(o.errs.trim().to_string());
                }
                Err(err) => {
                    success = false;
                    self.diagnostics.borrow_mut().report_error(err);
                }
                _ => {}
            }
        }

        success
    }

    fn accept_linkage(&self, res: ProcSpawnRusult) {
        match res {
            Ok(o) if o.exit_code != 0 => {
                self.diagnostics
                    .borrow_mut()
                    .report_error_str(o.errs.trim().to_string());
            }
            Ok(o) if !o.errs.is_empty() => {
                self.diagnostics
                    .borrow_mut()
                    .report_warning(o.errs.trim().to_string());
            }
            Err(err) => {
                self.diagnostics.borrow_mut().report_error(err);
            }
            _ => {}
        }
    }

    /// flags are ordered this way: <std> <cflags> <-I...> <-c file.cpp> <-o file.o>
    #[inline]
    fn full_cargs(&self, step: &Step) -> Option<Vec<String>> {
        let Step::Compilation {
            source,
            output,
            preset,
        } = step
        else {
            return None;
        };
        let mut res = Vec::default();
        let preset = self.ctx.config.presets.get(preset).unwrap();

        // <std>
        res.push(format!("-std=c++{}", self.ctx.config.std_as_str().unwrap()));
        // <cflags>
        res.extend_from_slice(&preset.cflags);
        // <-I...>
        self.ctx
            .config
            .include_dirs
            .iter()
            .filter_map(|p| p.to_str())
            .for_each(|s| res.push(format!("-I{s}")));
        // <-c file.cpp>
        res.extend_from_slice(&["-c".into(), source.clean().display().to_string()]);
        // <-o file.o>
        res.extend_from_slice(&["-o".into(), output.clean().display().to_string()]);

        Some(res)
    }

    /// flags are ordered this way: <std> <file.o...> <lflags> <-L...> <-l...> <-o file.exe>
    #[inline]
    fn full_largs(&self, step: &Step) -> Option<Vec<String>> {
        let Step::Linkage {
            source,
            output,
            preset,
        } = step
        else {
            return None;
        };
        let mut res = Vec::default();
        let preset = self.ctx.config.presets.get(preset).unwrap();

        // <std>
        res.push(format!("-std=c++{}", self.ctx.config.std_as_str().unwrap()));

        // <file.o...>
        let source: Vec<String> = source
            .iter()
            .map(|p| p.clean().display().to_string())
            .collect();
        res.extend_from_slice(&source);
        // <lflags>
        res.extend_from_slice(&preset.lflags);
        // <-L...>
        self.ctx
            .config
            .lib_dirs
            .iter()
            .filter_map(|p| p.to_str())
            .for_each(|s| res.push(format!("-L{s}")));
        // <-l...>
        preset.libs.iter().for_each(|s| res.push(format!("-l{s}")));
        // <-o file.exe>
        res.extend_from_slice(&["-o".into(), output.clean().display().to_string()]);

        Some(res)
    }
}

impl PlanVisitor for PlanExecutor<'_> {
    fn visit_compilation(&self, step: &Step) {
        let Some(args) = self.full_cargs(step) else {
            return;
        };

        let Step::Compilation { source, .. } = step else {
            return;
        };
        Logger::info(&format!(
            "Compilation started: {}",
            source.clean().display()
        ));

        let tx = self.compilation_tx.borrow();
        if tx.is_none() {
            return;
        }
        self.compilation_timer.start();
        ProcSpawner::spawn_into_pool(
            COMPILER.into(),
            args,
            &self.ctx.thread_pool,
            tx.as_ref().unwrap().clone(),
        );
    }

    fn visit_linkage(&self, step: &Step) {
        if !self.accept_compilation() {
            return;
        }
        let Some(args) = self.full_largs(step) else {
            return;
        };

        Logger::info("Linking executable");
        self.accept_linkage(ProcSpawner::spawn_and_wait(COMPILER, &args));
    }

    /// Will print all diagnostics before launch.
    fn visit_run(&self, step: &Step) {
        let Step::Run { exe, args } = step else {
            return;
        };

        if self.diagnostics.borrow().contains_error() {
            return;
        }
        self.diagnostics.borrow_mut().print_all_clear();
        Logger::info(&format!("Running: {exe}"));

        match ProcSpawner::spawn_into_parent(exe, args) {
            Ok(code) if code != 0 => {
                Logger::error(&format!("Program did not finish successfully: [{code}]"))
            }
            Err(err) => self.diagnostics.borrow_mut().report_error(err),
            _ => {}
        }
    }

    fn visit_make_dir(&self, step: &Step) {
        let Step::CreateDir { path } = step else {
            return;
        };
        if let Err(err) = self.fs_m.mkdir(path) {
            self.diagnostics.borrow_mut().report_error(err);
        }
    }

    fn visit_remove_dir(&self, step: &Step) {
        let Step::RemoveDir { path } = step else {
            return;
        };
        if let Err(err) = self.fs_m.delete(path) {
            self.diagnostics.borrow_mut().report_error(err);
        }
    }

    fn visit_make_file(&self, step: &Step) {
        let Step::CreateFile { path } = step else {
            return;
        };
        if let Err(err) = self.fs_m.mkfile(path) {
            self.diagnostics.borrow_mut().report_error(err);
        }
    }

    fn visit_remove_file(&self, step: &Step) {
        let Step::RemoveFile { path } = step else {
            return;
        };
        if let Err(err) = self.fs_m.delete(path) {
            self.diagnostics.borrow_mut().report_error(err);
        }
    }
}

/// Destructor waits for all parallel tasks.
impl Drop for PlanExecutor<'_> {
    fn drop(&mut self) {
        self.ctx.thread_pool.join();
    }
}
