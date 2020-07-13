use std::sync::Arc;

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, Error>;

#[allow(dead_code)]
pub fn general_error<T>(s:&'static str) -> Result<T> {
    Err(Error::General(s))
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Error {
    General(&'static str),
    IO(Arc<std::io::Error>),
    ParseFloat(Arc<std::num::ParseFloatError>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::General(message) => write!(f, "General({})", message),
            Error::IO(error) => write!(f, "IO({})", error),
            Error::ParseFloat(error) => write!(f, "ParseFloat({})", error),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::General(_) => None,
            Error::IO(error) => Some(error.as_ref()),
            Error::ParseFloat(error) => Some(error.as_ref()),
        }
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(Arc::new(error))
    }
}

impl std::convert::From<std::num::ParseFloatError> for Error {
    fn from(error: std::num::ParseFloatError) -> Self {
        Error::ParseFloat(Arc::new(error))
    }
}

