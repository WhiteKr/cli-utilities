use core::fmt;

#[derive(Debug)]
pub enum CliError {
    IoError(std::io::Error),
    InvalidArgument(String),
    CommandError(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::IoError(err) => write!(f, "IO Error: {}", err),
            CliError::InvalidArgument(msg) => write!(f, "Invalid Argument: {}", msg),
            CliError::CommandError(msg) => write!(f, "Command Error: {}", msg),
        }
    }
}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        CliError::IoError(err)
    }
}

pub type CliResult<T> = Result<T, CliError>;
