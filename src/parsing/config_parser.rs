use std::{cell::RefCell, fs, path::PathBuf, rc::Rc};

use crate::errors::ParsingError;

use super::config::{Config, DEFAULT_CONFIG_STR};

#[derive(Default)]
pub struct ConfigParser {
    file: PathBuf,

    cfg: Rc<RefCell<Config>>,
}

impl ConfigParser {
    pub fn new(file: PathBuf, cfg: Rc<RefCell<Config>>) -> Self {
        Self { file, cfg }
    }

    /// Write default config into Context
    pub fn make_default(&self) -> Result<(), ParsingError> {
        let mut cfg_bind = self.cfg.borrow_mut();
        *cfg_bind = Self::parse_from_str(DEFAULT_CONFIG_STR)?;

        if cfg_bind.std_as_str().is_none() {
            return Err(ParsingError::WrongStandart(cfg_bind.std));
        }
        Ok(())
    }

    /// Parse into Context only user-defined params form .toml file.
    /// Does nothing if file doesn't exist.
    pub fn try_incremental_parse(&self) -> Result<(), ParsingError> {
        if !self.file.exists() {
            return Ok(());
        }
        let cfg_bind = &mut self.cfg.borrow_mut();

        let cfg_user_defined_str = fs::read_to_string(&self.file)?;
        let cfg_user_defined = Self::parse_from_str(&cfg_user_defined_str)?;

        cfg_bind.incremental_merge(cfg_user_defined);
        if cfg_bind.std_as_str().is_none() {
            return Err(ParsingError::WrongStandart(cfg_bind.std));
        }

        Ok(())
    }

    fn parse_from_str(input: &str) -> Result<Config, ParsingError> {
        let cfg: Config = match toml::from_str(input) {
            Ok(cfg) => cfg,
            Err(toml_err) => {
                return Err(toml_err.into());
            }
        };

        Ok(cfg)
    }
}

pub mod tests {
    use crate::core::Context;

    use super::ConfigParser;
    use std::path::PathBuf;

    #[test]
    fn simple_cfg_parser_debug() {
        let mock_file = PathBuf::from("test_assets/test_config.toml");
        let mock_ctx = Context::default();
        let parser = ConfigParser::new(mock_file, mock_ctx.config.clone());

        parser.make_default().unwrap();
        parser.try_incremental_parse().unwrap();

        println!("Parsed config: {:#?}", mock_ctx.config.borrow());
    }
}
