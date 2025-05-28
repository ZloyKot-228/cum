use std::path::PathBuf;

use crate::{
    core::{Context, FilesystemManagerCell},
    drivers::{dependency_analyzer::DependencyAnalyzer, fs_manager::FilesystemManager},
    errors::{PlannerError, QueryError},
};

use super::args_specification::*;

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

        if IncrementalBuild.is_satisfied_by(&self.ctx.args) {
            self.plan_compilation(true)?;
            self.plan_linkage()?;
        } else if FullBuild.is_satisfied_by(&self.ctx.args) {
            self.plan_compilation(false)?;
            self.plan_linkage()?;
        }

        Ok(())
    }

    pub fn plan_compilation(&mut self, incremental: bool) -> Result<(), PlannerError> {
        let obj_files = self
            .fs_m
            .find_all_with_extension("o", &PathBuf::from("target/obj"));
        let mut src_files = self
            .fs_m
            .find_all_with_extension("cpp", &PathBuf::from("src"));
        let newest_obj = FilesystemManager::find_newest(&obj_files);
        // Generate full list of objects to link.
        self.create_obj_list(&src_files);

        if src_files.is_empty() || newest_obj.is_none() {
            return Ok(());
        }

        if incremental {
            let mut anayzer = DependencyAnalyzer::new(self.fs_m.clone(), &src_files);
            anayzer.generate_dependencies()?;
            // Retain dirty .cpp files.
            src_files = anayzer.get_dirty_src(newest_obj.as_ref().unwrap());
        }

        for file in src_files {
            self.ctx.plan.add_compilation(
                file.clone(),
                Self::src_to_obj(&file),
                self.preset.clone(),
            );
        }

        Ok(())
    }

    pub fn plan_linkage(&mut self) -> Result<(), PlannerError> {
        let executable_path = self.ctx.config.presets[&self.preset].target_folder.clone();
        let executable_name = PathBuf::from(&self.ctx.config.target_name);

        let executable = if cfg!(target_os = "windows") {
            executable_path.join(executable_name).with_extension("exe")
        } else {
            executable_path.join(executable_name)
        };

        self.ctx
            .plan
            .add_linkage(self.obj_list.clone(), executable, self.preset.clone());

        Ok(())
    }

    fn create_obj_list(&mut self, src_list: &[PathBuf]) {
        self.obj_list = src_list.iter().map(Self::src_to_obj).collect();
    }

    /// src/deps/dep1.cpp turns into target/obj/src.deps.dep1.o
    fn src_to_obj(path: &PathBuf) -> PathBuf {
        let dotted: String = path
            .with_extension("o")
            .components()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join(".");
        PathBuf::from(format!("target/obj/{dotted}"))
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
}
