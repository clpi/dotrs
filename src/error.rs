use std::{fmt::{self, Write}, error, io};
use crate::fmt::print;

pub type DotCliResult<T> = Result<T, DotCliError>;

#[derive(Debug)]
pub enum DotCliError {
    IoError(io::Error),
    ReadLineError(rustyline::error::ReadlineError),
    ReadConfigError(toml::ser::Error),
    WriteConfigError(toml::de::Error),
    ArgError,
    HttpError,
}
impl fmt::Display for DotCliError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::IoError(e) => format!("IO Error: {}", e),
            Self::ReadConfigError(e) => format!("Read conf error: {}", e),
            Self::ReadLineError(e) => format!("Readln error: {}", e),
            Self::WriteConfigError(e) => format!("Read conf error: {}", e),
            Self::HttpError => format!("{}", "Http Error"),
            Self::ArgError => format!("Arg Error: {}", "")
        };
        print::err(&msg).unwrap();
        Ok(())
    }
}

impl From<io::Error> for DotCliError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl error::Error for DotCliError {}
