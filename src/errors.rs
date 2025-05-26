use thiserror::Error;

// Parsing error
#[derive(Debug, Error)]
pub enum ParsingErrorKind {
    #[error("File IO error: {0}")]
    FileIO(#[from] std::io::Error),

    #[error("File '{filename}' has wrong toml structure at {line}:{column}")]
    TomlStructure {
        filename: String,
        line: usize,
        column: usize,
    },
}
