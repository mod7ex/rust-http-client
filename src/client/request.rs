use super::{ methods::Method };
use std::{collections::{HashMap}, ops::Range, fmt::Display, io::Write};

static BOUNDARY: &str = "X_HTTPCLIENT_BOUNDARY";

#[derive(Debug)]
pub struct Request {
    method: Method,
    headers: HashMap<String, String>,
    query: String,
    range: Option<Range<usize>>,
    body: Option<Vec<u8>>
}

impl Request {
    pub fn new() -> Self {
        Request {
            method: Method::GET,
            headers: HashMap::new(),
            query: String::new(),
            range: None,
            body: None
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

    pub fn get_range(&self) -> Option<&Range<usize>> {
        if let Some(b) = &self.range {
            Some(b)
        } else {
            None
        }
    }

    pub fn get_method(&self) -> &Method {
        &self.method
    }

    pub fn get_query(&self) -> &str {
        &self.query
    }

    pub fn get_body(&self) -> Option<&Vec<u8>> {
        if let Some(b) = &self.body {
            Some(b)
        } else {
            None
        }
    }

    pub fn get_content_length(&self) -> usize {
        if let Some(b) = &self.body {
            b.len()
        } else {
            0
        }
    }
}

impl Request {
    pub fn add_query<T: Display>(mut self, key: &str, value: T) -> Self {
        let mut item = String::new();
        if !self.get_query().is_empty() {
            item.push('&');
        }
        item.push_str(key);
        item.push('=');
        item.push_str(&value.to_string());
        
        self.query.push_str(&item);

        self
    }
}

impl Request {
    pub fn form_data(mut self) -> Self {
        self.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string()
        );

        self
    }

    pub fn add_form_data<T: Display>(mut self, key: &str, value: T) -> Self {
        let mut item = String::new();
        item.push_str(key);
        item.push('=');
        item.push_str(&value.to_string());

        if let Some(mut body) = self.body {
            item.insert(0, '&');
            body.write_all(item.as_bytes()).unwrap();
            self.body = Some(body)
        } else {
            self.body = Some(item.as_bytes().to_vec())
        }

        self
    }
}

impl Request {
    pub fn multipart(mut self) -> Self {
        self.headers.insert(
            "Content-Type".to_string(),
            format!("multipart/form-data; boundary={}", BOUNDARY)
        );

        self
    }

    fn start_body_part(mut self) -> Self {
        let form_part_start = format!("--{}\r\n", BOUNDARY);

        let mut b = form_part_start.as_bytes();

        if let Some(mut body) = self.body {
            body.write_all(&b).unwrap();
            self.body = Some(body.to_owned());
        } else {
            self.body = Some(b.to_owned());
        }

        self
    }

    // https://stackoverflow.com/a/23517227/13278193

    /**
     * --X_HTTPCLIENT_BOUNDARY
     * Content-Disposition: form-data; name="name"
     * 
     * value
     * --X_HTTPCLIENT_BOUNDARY
     * Content-Disposition: form-data; name="other_name"
     * 
     * other value
     * --X_HTTPCLIENT_BOUNDARY--
     */

    pub fn add_data<T: Display>(mut self, key: &str, value: T) -> Self {
        self = self.start_body_part();

        let mut body = self.body.unwrap(); // body 100% exists 

        let mut item = String::new();
        item.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
        item.push_str(&format!("{}\r\n", value.to_string())); 

        body.write_all(item.as_bytes()).unwrap();

        self.body = Some(body);

        self
    }

    pub fn add_file(mut self, key: &str, filename: &str, file: Vec<u8>) -> Self {
        self = self.start_body_part();

        let mut body = self.body.unwrap(); // body 100% exists 

        let mut item = format!("Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n\r\n", key, filename);
        body.write_all(item.as_bytes()).unwrap();

        body.write_all(file.as_slice()).unwrap();
        body.write_all("\r\n".as_bytes());

        self.body = Some(body);

        self
    }

    pub fn close_multipart_form_data(mut self) -> Self {
        let mut body = self.body.unwrap(); // expects body to be filled

        let form_part_end = format!("--{}--", BOUNDARY);

        body.write_all(form_part_end.as_bytes()).unwrap();

        self.body = Some(body);

        self
    }
}