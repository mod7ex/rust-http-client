#[derive(Debug)]
pub enum Error {
    UrlParsingError,
    IoError
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UrlParsingError => write!(f, "Error parsing URL"),
            Error::IoError => write!(f, "IO operation error"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::IoError
    }
}