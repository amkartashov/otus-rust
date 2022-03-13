use std::sync::Arc;

use thiserror::Error as thiserrorError;

#[derive(thiserrorError, Debug, Clone)]
#[non_exhaustive]
pub enum Error {
    #[error("`{0}`")]
    Error(String),
    #[error("Device io error: {0}")]
    DeviceIOError(Arc<std::io::Error>),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Error(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        s.to_string().into()
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::DeviceIOError(Arc::new(e))
    }
}
