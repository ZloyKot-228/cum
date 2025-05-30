use std::{fs, path::PathBuf};

use crate::errors::ParsingError;

use super::config::{Config, DEFAULT_CONFIG_STR};

pub struct ConfigParser<'a> {
    file: PathBuf,

    cfg: &'a mut Config,
}

impl<'a> ConfigParser<'a> {
    pub fn new(file: PathBuf, cfg: &'a mut Config) -> Self {
        Self { file, cfg }
    }

    /// Write default config into Context
    pub fn make_default(&mut self) -> Result<(), ParsingError> {
        *self.cfg = Self::parse_from_str(DEFAULT_CONFIG_STR)?;

        if self.cfg.std_as_str().is_none() {
            return Err(ParsingError::WrongStandart(self.cfg.std));
        }
        self.cfg.normalize_pathes();

        Ok(())
    }

    /// Parse into Context only user-defined params form .toml file.
    /// Does nothing if file doesn't exist.
    pub fn try_incremental_parse(&mut self) -> Result<(), ParsingError> {
        if !self.file.exists() {
            return Ok(());
        }

        let cfg_user_defined_str = fs::read_to_string(&self.file)?;
        let cfg_user_defined = Self::parse_from_str(&cfg_user_defined_str)?;

        self.cfg.incremental_merge(cfg_user_defined);
        if self.cfg.std_as_str().is_none() {
            return Err(ParsingError::WrongStandart(self.cfg.std));
        }
        self.cfg.normalize_pathes();

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
        let mock_file = PathBuf::from("test_assets/Cum.toml");
        let mut mock_ctx = Context::default();
        let mut parser = ConfigParser::new(mock_file, &mut mock_ctx.config);

        parser.make_default().unwrap();
        parser.try_incremental_parse().unwrap();

        println!("Parsed config: {:#?}", mock_ctx.config);
    }
}
