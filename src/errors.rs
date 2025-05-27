use thiserror::Error;

// Parsing error
#[derive(Debug, Error)]
pub enum ParsingError {
    #[error("Toml parsing error: {0}")]
    TomlParsing(#[from] toml::de::Error),

    #[error("Unallowed standart: '{0}'")]
    WrongStandart(u8),

    #[error("Parameter required: '{0}'")]
    ParamRequired(String),

    #[error("File IO error: {0}")]
    FileIO(#[from] std::io::Error),
}

// Query error
#[derive(Debug, Error)]
pub enum QueryError {
    #[error("Unknown command: '{0}'")]
    UnknownCommand(String),

    #[error("No arguments provided")]
    NoArgs,
}
