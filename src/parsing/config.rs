use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

pub const CONFIG_FILE_PATH: &str = "./Cum.toml";
pub const DEFAULT_CONFIG_STR: &str = include_str!("../../assets/default_config.toml");

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub std: u8,

    #[serde(default)]
    pub include_dirs: Vec<PathBuf>,

    #[serde(default)]
    pub lib_dirs: Vec<PathBuf>,

    #[serde(default)]
    pub target_name: String,

    #[serde(default)]
    pub presets: HashMap<String, Preset>,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Preset {
    #[serde(default)]
    pub cflags: Vec<String>,

    #[serde(default)]
    pub lflags: Vec<String>,

    #[serde(default)]
    pub libs: Vec<String>,

    #[serde(default)]
    pub target_folder: PathBuf,
}

impl Config {
    pub fn incremental_merge(&mut self, other: Config) {
        if other.std != 0 {
            self.std = other.std;
        }

        if !other.include_dirs.is_empty() {
            self.include_dirs = other.include_dirs;
        }
        if !other.lib_dirs.is_empty() {
            self.lib_dirs = other.lib_dirs;
        }
        if !other.target_name.is_empty() {
            self.target_name = other.target_name;
        }

        for (key, value) in other.presets {
            if !self.presets.contains_key(&key) {
                self.presets.insert(key, value);
                continue;
            }

            let preset = self.presets.get_mut(&key).unwrap();
            if !value.cflags.is_empty() {
                preset.cflags = value.cflags.clone();
            }
            if !value.lflags.is_empty() {
                preset.lflags = value.lflags.clone();
            }
            if !value.libs.is_empty() {
                preset.libs = value.libs.clone();
            }
            if value.target_folder != PathBuf::default() {
                preset.target_folder = value.target_folder.clone();
            }
        }
    }

    pub fn std_as_str(&self) -> Option<String> {
        match self.std {
            3 => Some("03".to_string()),
            11 => Some(self.std.to_string()),
            14 => Some(self.std.to_string()),
            17 => Some(self.std.to_string()),
            20 => Some(self.std.to_string()),
            23 => Some(self.std.to_string()),
            _ => None,
        }
    }
}
