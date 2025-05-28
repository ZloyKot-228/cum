use std::path::{Path, PathBuf};

use crate::{core::FilesystemManagerCell, errors::ExecutionError};

pub struct DependencyAnalyzer<'a> {
    fs_m: FilesystemManagerCell,
    pub src_files: &'a Vec<PathBuf>,
    pub dependency_spans: Vec<DependencySpan<'a>>,
}

pub struct DependencySpan<'a> {
    dependent: &'a Path,
    dependencies: Vec<PathBuf>,
}

impl<'a> DependencyAnalyzer<'a> {
    pub fn new(fs_m: FilesystemManagerCell, src_files: &'a Vec<PathBuf>) -> Self {
        Self {
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

    fn push_dependency(&mut self, file: &Path) -> Result<(), ExecutionError> {
        todo!()
    }
}
