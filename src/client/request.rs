use super::{ methods::Method };
use std::{collections::{HashMap}, ops::Range};

pub struct Request {
    method: Method,
    headers: HashMap<String, String>,
    range: Option<Range<usize>>
}

impl Request {
    pub fn new() -> Self {
        Request {
            method: Method::GET,
            headers: HashMap::new(),
            range: None
        }
    }

    pub fn set_method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    pub fn set_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    pub fn set_range(mut self, range: Range<usize>) -> Self {
        self.range = Some(range);
        self
    }

    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn get_range(&self) -> &Option<Range<usize>> {
        &self.range
    }

    pub fn get_method(&self) -> &Method {
        &self.method
    }
}