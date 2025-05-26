use std::{fmt::Display, path::PathBuf};

pub const CONFIG_FILE_PATH: &str = "./Cum.toml";

#[derive(Default)]
pub struct Config {}

pub struct Preset {
    pub cflags: Vec<String>,
    pub lflags: Vec<String>,
    pub libs: Vec<String>,
    pub target_folder: PathBuf,
}

// Displays
impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
