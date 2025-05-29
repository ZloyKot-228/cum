use std::{
    env::{self},
    path::{Path, PathBuf},
};

use crate::{
    core::{Context, FilesystemManagerCell},
    drivers::fs_manager::FilesystemManager,
    parsing::{
        arg_parser::{ArgParser, Args},
        config::Config,
        config_parser::ConfigParser,
    },
};

pub struct MockFactory;

impl MockFactory {
    pub fn mock_cfg_default() -> Config {
        let mut res = Config::default();
        let mut parser = ConfigParser::new(PathBuf::default(), &mut res);
        parser.make_default().unwrap();
        res
    }

    pub fn mock_fs_m() -> FilesystemManagerCell {
        FilesystemManager::new("test_assets/".into())
            .unwrap()
            .into()
    }

    pub fn mock_args(args: &[&str]) -> Args {
        let mut res = Args::default();
        let mut parser = ArgParser::new(args.iter().copied().map(String::from).collect(), &mut res);
        parser.try_parse().unwrap();
        res
    }

    pub fn mock_ctx_for_call(args: &[&str]) -> Context {
        let mut ctx = Context::default();
        ctx.config = Self::mock_cfg_default();
        ctx.args = Self::mock_args(args);
        ctx
    }
}

#[inline]
pub fn set_dir_to_tests() {
    if env::current_dir()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        != "test_assets"
    {
        env::set_current_dir(Path::new("test_assets/")).unwrap();
    }
}
