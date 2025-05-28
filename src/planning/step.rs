use std::path::PathBuf;

use super::plan::PlanVisitor;

#[derive(Debug)]
pub enum Step {
    Compilation {
        source: PathBuf,
        output: PathBuf,
        preset: String,
    },
    Linkage {
        source: Vec<PathBuf>,
        output: PathBuf,
        preset: String,
    },
    CreateDir {
        path: PathBuf,
    },
    RemoveDir {
        path: PathBuf,
    },
    CreateFile {
        path: PathBuf,
    },
    RemoveFile {
        path: PathBuf,
    },
}

impl Step {
    pub fn accept<V: PlanVisitor>(&self, visitor: &V) {
        match self {
            Step::Compilation { .. } => visitor.visit_compilation(self),
            Step::Linkage { .. } => visitor.visit_linkage(self),
            Step::CreateDir { .. } => visitor.visit_make_dir(self),
            Step::RemoveDir { .. } => visitor.visit_remove_dir(self),
            Step::CreateFile { .. } => visitor.visit_make_file(self),
            Step::RemoveFile { .. } => visitor.visit_remove_file(self),
        }
    }
}
