use std::path::{Path, PathBuf};

use path_clean::PathClean;

use crate::{
    core::{Context, FilesystemManagerCell},
    drivers::{dependency_analyzer::DependencyAnalyzer, fs_manager::FilesystemManager},
    errors::{PlannerError, QueryError},
    logger::Logger,
};

use super::{args_specification::*, step::Step};

pub struct Planner<'a> {
    ctx: &'a mut Context,
    fs_m: FilesystemManagerCell,

    obj_list: Vec<PathBuf>,
    preset: String,
}

impl<'a> Planner<'a> {
    pub fn new(ctx: &'a mut Context, fs_m: FilesystemManagerCell) -> Self {
        Self {
            ctx,
            fs_m,
            preset: String::default(),
            obj_list: Vec::default(),
        }
    }

    pub fn try_make_plan(&mut self) -> Result<(), PlannerError> {
        self.get_preset()?;

        if IncrementalBuild
            .or(IncrementalRun)
            .is_satisfied_by(&self.ctx.args)
        {
            Logger::info("Analyzing dependencies...");
            self.plan_compilation(true)?;
            self.plan_linkage();
        } else if FullBuild.or(FullRun).is_satisfied_by(&self.ctx.args) {
            self.plan_compilation(false)?;
            self.plan_linkage();
        } else if InitProject.is_satisfied_by(&self.ctx.args) {
            Logger::info("Initializiing empty project...");
            self.plan_init();
        } else if let Some(cmd) = &self.ctx.args.command.as_ref() {
            return Err(QueryError::UnknownCommand(cmd.to_string()).into());
        }

        if FullRun.or(IncrementalRun).is_satisfied_by(&self.ctx.args) {
            self.plan_run_linked();
        }

        Ok(())
    }

    fn plan_compilation(&mut self, mut incremental: bool) -> Result<(), PlannerError> {
        let obj_files = self
            .fs_m
            .find_all_with_extension("o", &PathBuf::from("target/obj"));
        let mut src_files = self
            .fs_m
            .find_all_with_extension("cpp", &PathBuf::from("src"));
        // Generate full list of objects to link.
        self.create_obj_list(&src_files);

        if src_files.is_empty() {
            Logger::info("No .cpp files found");
            return Ok(());
        }
        if obj_files.is_empty() {
            incremental = false;
        }

        if incremental {
            let mut anayzer =
                DependencyAnalyzer::new(&self.ctx.config, self.fs_m.clone(), &src_files);
            anayzer.generate_dependencies()?;
            // Retain .cpp files that need to be recompiled.
            src_files = anayzer.get_dirty_src();
        }

        for file in src_files {
            self.ctx.plan.add_compilation(
                file.clone(),
                FilesystemManager::src_to_obj(&file),
                self.preset.clone(),
            );
        }

        Ok(())
    }

    fn plan_linkage(&mut self) {
        let executable_path = self.ctx.config.presets[&self.preset].target_folder.clone();
        let executable_name = PathBuf::from(&self.ctx.config.target_name);

        let executable = if cfg!(target_os = "windows") {
            executable_path.join(executable_name).with_extension("exe")
        } else {
            executable_path.join(executable_name)
        };

        if !executable.parent().map(Path::exists).unwrap_or(true) {
            self.ctx
                .plan
                .add_make_dir(executable.parent().unwrap().into());
        }

        self.ctx
            .plan
            .add_linkage(self.obj_list.clone(), executable, self.preset.clone());
    }

    fn create_obj_list(&mut self, src_list: &[PathBuf]) {
        self.obj_list = src_list.iter().map(FilesystemManager::src_to_obj).collect();
    }

    fn get_preset(&mut self) -> Result<(), QueryError> {
        let preset = self
            .ctx
            .args
            .named_params
            .get("preset")
            .cloned()
            .unwrap_or(String::from("debug"));

        if !self.ctx.config.presets.contains_key(&preset) {
            Err(QueryError::InvalidPreset(preset))
        } else {
            self.preset = preset;
            Ok(())
        }
    }

    /// Default project structure is defined here.
    fn plan_init(&mut self) {
        self.ctx.plan.add_make_file("Cum.toml".into());
        self.ctx.plan.add_make_file("src/main.cpp".into());
        self.ctx.plan.add_make_file("tests/test_runner.cpp".into());
        self.ctx.plan.add_make_dir("target/obj".into());
        self.ctx.plan.add_make_dir("include".into());
        self.ctx.plan.add_make_dir("dependencies/include".into());
        self.ctx.plan.add_make_dir("dependencies/lib".into());
    }

    fn plan_run_linked(&mut self) {
        if let Some(Step::Linkage { output, .. }) = self.ctx.plan.steps().last().cloned() {
            self.ctx.plan.add_run(
                output.clean().display().to_string(),
                self.ctx.args.freestanding_params.clone(),
            );
        }
    }
}

pub mod tests {
    use crate::{
        core::{Context, FilesystemManagerCell},
        test_utils::{set_dir_to_tests, MockFactory},
    };

    use super::Planner;

    #[test]
    fn simple_planner_build_inc_debug() {
        set_dir_to_tests();
        let fs_m = FilesystemManagerCell::default();
        let mut mock_ctx = MockFactory::mock_ctx_for_call(&["cum.exe", "build"]);

        let mut planner = Planner::new(&mut mock_ctx, fs_m);
        planner.try_make_plan().unwrap();

        println!("IncrementalBuild: {:#?}", mock_ctx.plan);
    }

    #[test]
    fn simple_planner_run_inc_debug() {
        set_dir_to_tests();
        let fs_m = FilesystemManagerCell::default();
        let mut mock_ctx =
            MockFactory::mock_ctx_for_call(&["cum.exe", "run", "--", "-param", "-flag"]);

        let mut planner = Planner::new(&mut mock_ctx, fs_m);
        planner.try_make_plan().unwrap();

        println!("IncrementalRun: {:#?}", mock_ctx.plan);
    }
}
