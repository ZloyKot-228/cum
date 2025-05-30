use std::path::PathBuf;

use thiserror::Error;

// Parsing error
#[derive(Debug, Error)]
pub enum ParsingError {
    #[error("Toml parsing error: [{0}]")]
    TomlParsing(#[from] toml::de::Error),

    #[error("Unallowed standart: '{0}'")]
    WrongStandart(u8),

    #[error("Parameter required: '{0}'")]
    ParamRequired(String),

    #[error("File IO error: [{0}]")]
    FileIO(#[from] std::io::Error),
}

// Query error
#[derive(Debug, Error)]
pub enum QueryError {
    #[error("Unknown command: '{0}'")]
    UnknownCommand(String),

    #[error("Invalid preset: '{0}'")]
    InvalidPreset(String),

    #[error("Invalid entry point: '{0}'")]
    InvalidEntryPoint(PathBuf),

    #[error("No arguments provided")]
    NoArgs,
}

// Planner error
#[derive(Debug, Error)]
pub enum PlannerError {
    #[error("Query error: [{0}]")]
    QueryError(#[from] QueryError),

    #[error("Execution error: [{0}]")]
    ExecutionError(#[from] ExecutionError),
}

// Execution Error
#[derive(Debug, Error)]
pub enum ExecutionError {
    #[error("Process IO error: [{0}]")]
    ProcIO(#[from] std::io::Error),

    #[error("Process finished with error ({code}): {errs}")]
    ProcErr { code: i32, errs: String },
}
