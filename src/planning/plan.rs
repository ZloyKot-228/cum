use std::path::PathBuf;

use super::step::Step;

pub trait PlanVisitor {
    fn visit_compilation(&self, step: &Step);
    fn visit_linkage(&self, step: &Step);
    fn visit_make_dir(&self, step: &Step);
    fn visit_remove_dir(&self, step: &Step);
    fn visit_make_file(&self, step: &Step);
    fn visit_remove_file(&self, step: &Step);
}

#[derive(Default)]
pub struct Plan {
    steps: Vec<Step>,
}

impl Plan {
    pub fn add_compilation(&mut self, source: PathBuf, output: PathBuf, preset: String) {
        self.steps.push(Step::Compilation {
            source,
            output,
            preset,
        });
    }

    pub fn add_linkage(&mut self, source: Vec<PathBuf>, output: PathBuf, preset: String) {
        self.steps.push(Step::Linkage {
            source,
            output,
            preset,
        });
    }

    pub fn add_make_dir(&mut self, path: PathBuf) {
        self.steps.push(Step::CreateDir { path });
    }

    pub fn add_remove_dir(&mut self, path: PathBuf) {
        self.steps.push(Step::RemoveDir { path });
    }

    pub fn add_make_file(&mut self, path: PathBuf) {
        self.steps.push(Step::CreateFile { path });
    }

    pub fn add_remove_file(&mut self, path: PathBuf) {
        self.steps.push(Step::RemoveFile { path });
    }

    pub fn steps(&self) -> &Vec<Step> {
        &self.steps
    }
}
