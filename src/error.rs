#[derive(Debug)]
pub enum Error {
    IoError(String),
    UrlParsingError(String),
    Utf8PartingError(String),
    HeaderParsingError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UrlParsingError(v) => write!(f, "Error parsing URL: {}", v),
            Error::IoError(v) => write!(f, "IO operation error: {}", v),
            Error::Utf8PartingError(v) => write!(f, "Error while parsing UTF8: {}", v),
            Error::HeaderParsingError(v) => write!(f, "Error while parsing header: {}", v),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        let msg = e.to_string();
        Error::IoError(msg)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        let msg = e.to_string();
        Error::Utf8PartingError(msg)
    }
}