use std::collections::{HashMap, HashSet};

use crate::{core::ContextCell, errors::QueryError};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Args {
    /// First argument from command line (init, build, etc.).
    pub command: Option<String>,
    /// Params which start with '-'.
    pub flags: HashSet<String>,
    /// Params which start with '--' (--preset=some). Counts as a flag.
    pub named_params: HashMap<String, String>,
    pub unnamed_params: Vec<String>,
    /// Params which go after '--' param.
    pub freestanding_params: Vec<String>,
}

pub struct ArgParser {
    args: Vec<String>,
    ctx: ContextCell,
}

impl ArgParser {
    pub fn new(args: Vec<String>, ctx: ContextCell) -> Self {
        Self { args, ctx }
    }

    pub fn try_parse(&self) -> Result<(), QueryError> {
        if self.args.len() < 2 {
            return Err(QueryError::NoArgs);
        }

        let args_bind = &mut self.ctx.borrow_mut().args;
        for (i, arg) in self.args.iter().skip(1).enumerate() {
            // Consume freestanding_params
            if arg == "--" {
                args_bind
                    .freestanding_params
                    .extend_from_slice(&self.args[i + 2..]);
                break;
            }

            if Self::is_named(arg) {
                let pair = Self::get_named_from(arg);
                args_bind.named_params.insert(pair.0, pair.1);
                continue;
            } else if Self::is_flag(arg) {
                args_bind.flags.insert(Self::get_flag_from(arg));
                continue;
            }

            // Handle unnamed param or command
            if i == 0 {
                args_bind.command = Some(arg.clone());
            } else {
                args_bind.unnamed_params.push(arg.clone());
            }
        }

        Ok(())
    }

    #[inline]
    fn is_flag(arg: &str) -> bool {
        arg.starts_with("-") && !arg.starts_with("--")
    }

    #[inline]
    fn is_named(arg: &str) -> bool {
        arg.starts_with("--")
    }

    #[inline]
    fn get_named_from(arg: &str) -> (String, String) {
        let mut pair: (String, String) = (String::new(), String::new());
        // --config=some
        //   ^     ^
        //   OR
        // --config
        //   ^     ^
        if let Some(eq_pos) = arg.find('=') {
            pair.0 = String::from(&arg[2..eq_pos]);
            pair.1 = String::from(&arg[eq_pos + 1..]);
        } else {
            pair.0 = String::from(&arg[2..]);
            pair.1 = pair.0.clone();
        }

        pair
    }

    #[inline]
    fn get_flag_from(arg: &str) -> String {
        String::from(&arg[1..])
    }
}

impl Args {
    pub fn have_flag(&self, flag: &str) -> bool {
        self.flags.contains(flag) || self.named_params.contains_key(flag)
    }
}

pub mod tests {

    use crate::{core::ContextCell, parsing::arg_parser::Args};

    use super::ArgParser;

    #[test]
    fn simple_args_regression() {
        let mock_args: Vec<String> = vec![
            "cum.exe",
            "build",
            "--config=release",
            "-v",
            "main.cpp",
            "--",
            "--forward-flag",
        ]
        .iter()
        .map(|s| String::from(*s))
        .collect();
        let mock_ctx = ContextCell::default();
        let parser = ArgParser::new(mock_args, mock_ctx.clone());

        let mut expected_args = Args::default();
        expected_args.command = Some("build".to_string());
        expected_args.flags.insert("v".to_string());
        expected_args
            .named_params
            .insert(String::from("config"), String::from("release"));
        expected_args.unnamed_params.push(String::from("main.cpp"));
        expected_args
            .freestanding_params
            .push(String::from("--forward-flag"));

        parser.try_parse().unwrap();

        println!("Parsed args: {:#?}", mock_ctx.borrow().args);
        assert_eq!(mock_ctx.borrow().args, expected_args);
    }

    #[test]
    fn simple_args_param_as_flag() {
        let mock_args: Vec<String> = vec!["cum.exe", "--help"]
            .iter()
            .map(|s| String::from(*s))
            .collect();
        let mock_ctx = ContextCell::default();
        let parser = ArgParser::new(mock_args, mock_ctx.clone());

        let mut expected_args = Args::default();
        expected_args
            .named_params
            .insert(String::from("help"), String::from("help"));

        parser.try_parse().unwrap();

        assert_eq!(mock_ctx.borrow().args, expected_args);
    }
}
