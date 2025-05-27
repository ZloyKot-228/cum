use std::{fs, path::PathBuf};

use crate::{core::ContextCell, errors::ParsingError};

use super::config::{Config, DEFAULT_CONFIG_STR};

#[derive(Default)]
pub struct ConfigParser {
    file: PathBuf,

    ctx: ContextCell,
}

impl ConfigParser {
    pub fn new(file: PathBuf, ctx: ContextCell) -> Self {
        Self { file, ctx }
    }

    /// Write default config into Context
    pub fn make_default(&self) -> Result<(), ParsingError> {
        let mut ctx_bind = self.ctx.borrow_mut();
        ctx_bind.config = Self::parse_from_str(DEFAULT_CONFIG_STR)?;

        if ctx_bind.config.std_as_str().is_none() {
            return Err(ParsingError::WrongStandart(ctx_bind.config.std));
        }
        Ok(())
    }

    /// Parse into Context only user-defined params form .toml file.
    /// Does nothing if file doesn't exist.
    pub fn try_incremental_parse(&self) -> Result<(), ParsingError> {
        if !self.file.exists() {
            return Ok(());
        }
        let cfg_bind = &mut self.ctx.borrow_mut().config;

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
    use super::ConfigParser;
    use crate::core::ContextCell;
    use std::path::PathBuf;

    #[test]
    fn simple_cfg_parser_debug() {
        let mock_file = PathBuf::from("test_assets/test_config.toml");
        let mock_ctx = ContextCell::default();
        let parser = ConfigParser::new(mock_file, mock_ctx.clone());

        parser.make_default().unwrap();
        parser.try_incremental_parse().unwrap();

        println!("Parsed config: {:#?}", mock_ctx.borrow().config);
    }
}
