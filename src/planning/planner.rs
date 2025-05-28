use std::{cell::RefCell, path::PathBuf, rc::Rc};

use crate::{
    core::FilesystemManagerCell,
    drivers::{dependency_analyzer::DependencyAnalyzer, fs_manager::FilesystemManager},
    errors::{PlannerError, QueryError},
    parsing::{arg_parser::Args, config::Config},
};

use super::{args_specification::*, plan::Plan};

pub struct Planner {
    cfg: Rc<RefCell<Config>>,
    args: Rc<RefCell<Args>>,
    plan: Rc<RefCell<Plan>>,
    fs_m: FilesystemManagerCell,

    obj_list: RefCell<Vec<PathBuf>>,
    preset: RefCell<String>,
}

impl Planner {
    pub fn new(
        cfg: Rc<RefCell<Config>>,
        args: Rc<RefCell<Args>>,
        plan: Rc<RefCell<Plan>>,
        fs_m: FilesystemManagerCell,
    ) -> Self {
        Self {
            cfg,
            args,
            plan,
            fs_m,
            preset: String::default().into(),
            obj_list: Vec::default().into(),
        }
    }

    pub fn try_make_plan(&self) -> Result<(), PlannerError> {
        self.get_preset()?;
        let args_bind = self.args.borrow();

        if IncrementalBuild.is_satisfied_by(&args_bind) {
            self.plan_compilation(true)?;
            self.plan_linkage()?;
        } else if FullBuild.is_satisfied_by(&args_bind) {
            self.plan_compilation(false)?;
            self.plan_linkage()?;
        }

        Ok(())
    }

    pub fn plan_compilation(&self, incremental: bool) -> Result<(), PlannerError> {
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

        let mut plan_bind = self.plan.borrow_mut();
        for file in src_files {
            plan_bind.add_compilation(
                file.clone(),
                Self::src_to_obj(&file),
                self.preset.borrow().clone(),
            );
        }

        Ok(())
    }

    pub fn plan_linkage(&self) -> Result<(), PlannerError> {
        let cfg_bind = self.cfg.borrow();
        let preset_bind = self.preset.borrow();

        let executable_path = cfg_bind.presets[preset_bind.as_str()].target_folder.clone();
        let executable_name = PathBuf::from(&cfg_bind.target_name);

        let executable = if cfg!(target_os = "windows") {
            executable_path.join(executable_name).with_extension("exe")
        } else {
            executable_path.join(executable_name)
        };

        self.plan.borrow_mut().add_linkage(
            self.obj_list.borrow().clone(),
            executable,
            preset_bind.clone(),
        );

        Ok(())
    }

    fn create_obj_list(&self, src_list: &[PathBuf]) {
        let mut list_bind = self.obj_list.borrow_mut();
        *list_bind = src_list.iter().map(Self::src_to_obj).collect();
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

    fn get_preset(&self) -> Result<(), QueryError> {
        let preset = self
            .args
            .borrow()
            .named_params
            .get("preset")
            .cloned()
            .unwrap_or(String::from("debug"));

        if !self.cfg.borrow().presets.contains_key(&preset) {
            Err(QueryError::InvalidPreset(preset))
        } else {
            *self.preset.borrow_mut() = preset;
            Ok(())
        }
    }
}
