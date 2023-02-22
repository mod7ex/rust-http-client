use std::fmt::{ Display, Formatter, Result };

pub enum Method {
    GET,
    PUT,
    PATCH,
    POST,
    HEAD,
    OPTIONS,
    DELETE,
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::PUT => write!(f, "PUT"),
            Method::PATCH => write!(f, "PATCH"),
            Method::POST => write!(f, "POST"),
            Method::HEAD => write!(f, "HEAD"),
            Method::OPTIONS => write!(f, "OPTIONS"),
            Method::DELETE => write!(f, "DELETE"),
        }
    }
}