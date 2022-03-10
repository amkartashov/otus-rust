use thiserror::Error as thiserrorError;

#[derive(thiserrorError, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("`{0}`")]
    Error(String),
    #[error("Device io error: {0}")]
    DeviceIOError(#[from] std::io::Error),
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
