#![allow(dead_code, unused_variables)]
use std::path::{Path, PathBuf};

use crate::{core::FilesystemManagerCell, errors::ExecutionError, parsing::config::Config};

use super::proc_spawner::ProcSpawner;

pub struct DependencyAnalyzer<'a> {
    fs_m: FilesystemManagerCell,
    cfg: &'a Config,

    pub src_files: &'a Vec<PathBuf>,
    pub dependency_spans: Vec<DependencySpan<'a>>,
}

#[derive(Debug)]
pub struct DependencySpan<'a> {
    dependent: &'a Path,
    dependencies: Vec<PathBuf>,
}

pub struct MakefileParser;

// TODO: Distribute analyzis on thread pool.
impl<'a> DependencyAnalyzer<'a> {
    pub fn new(cfg: &'a Config, fs_m: FilesystemManagerCell, src_files: &'a Vec<PathBuf>) -> Self {
        Self {
            cfg,
            fs_m,
            src_files,
            dependency_spans: Vec::default(),
        }
    }

    pub fn generate_dependencies(&mut self) -> Result<(), ExecutionError> {
        for file in self.src_files {
            self.push_dependency(file)?;
        }
        Ok(())
    }

    /// Get entries from src_files which need to be recompiled.
    /// 'origin' is file that represents last compilation time.
    pub fn get_dirty_src(&self, origin: &Path) -> Vec<PathBuf> {
        todo!()
    }

    fn push_dependency(&mut self, file: &'a Path) -> Result<(), ExecutionError> {
        let clang_output = ProcSpawner::spawn_and_wait("clang++", &self.get_clang_args(file))?;
        if clang_output.exit_code != 0 {
            return Err(ExecutionError::ProcErr {
                code: clang_output.exit_code,
                errs: clang_output.errs,
            });
        }
        self.dependency_spans
            .push(MakefileParser::make_dependency(file, clang_output.outs));

        Ok(())
    }

    #[inline]
    fn get_clang_args(&self, file: &Path) -> Vec<String> {
        let mut res = vec![
            format!("-std=c++{}", self.cfg.std_as_str().unwrap()),
            "-MM".into(),
            file.display().to_string(),
        ];
        for dir in &self.cfg.include_dirs {
            res.push(format!("-I{}", dir.display()));
        }
        res
    }
}

impl MakefileParser {
    /// file: dependent,
    /// str: dependencies in Makefile format.
    pub fn make_dependency(file: &Path, str: String) -> DependencySpan {
        let dependencies = str
            .lines()
            .flat_map(|line| {
                let line = line.trim_end();
                let line = line.trim_end_matches('\\').trim();
                let mut parts: Vec<&str> = line.split_whitespace().collect();

                if parts.first().map(|s| s.ends_with(':')).unwrap_or(false) {
                    parts.remove(0);
                }
                parts.into_iter()
            })
            .map(PathBuf::from)
            .collect();

        DependencySpan {
            dependent: file,
            dependencies,
        }
    }
}

pub mod tests {
    use super::DependencyAnalyzer;
    use crate::{
        core::FilesystemManagerCell,
        parsing::config_parser::ConfigParser,
        test_utils::{set_dir_to_tests, MockFactory},
    };
    use std::path::PathBuf;

    #[test]
    fn simple_dep_anal_debug() {
        set_dir_to_tests();
        let fs_m = FilesystemManagerCell::default();
        let mock_cfg = MockFactory::mock_cfg_default();
        let mock_files: Vec<PathBuf> = vec!["src/main.cpp".into(), "src/dep1.cpp".into()];
        let mut analyzer = DependencyAnalyzer::new(&mock_cfg, fs_m, &mock_files);

        analyzer.generate_dependencies().unwrap();

        println!("Generated: {:#?}", analyzer.dependency_spans);
    }
}
