use std::collections::HashMap;

use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;

use crate::error::Error;

#[derive(Debug)]
pub struct Response {
    pub version: String,
    pub status_code: usize,
    pub status_name: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>
}

impl Response {
    pub async fn new(stream: &mut TcpStream) -> Result<Self, Error> {
        let mut buffer = Vec::new();
        let mut headers = HashMap::new();
        let mut body: Option<Vec<u8>> = None;
        let mut response_info = String::new();
        let mut is_header_section = false;

        loop {
            match stream.read_u8().await {
                Ok(bytes) => {
                    buffer.push(bytes);

                    if bytes as char != '\n' {
                        continue;
                        // we're collecting chars in the current line
                    }

                    if response_info.is_empty() {
                        response_info = String::from_utf8(buffer[..buffer.len() - 2].to_vec())?;
                        buffer.clear();
                        is_header_section = true;
                        continue;
                    }

                    if is_header_section {
                        if buffer.len() == 2 && buffer[0] as char == '\r' {
                            buffer.clear();
                            is_header_section = false;
                            continue;
                        }

                        let header_line = String::from_utf8(buffer[..buffer.len() - 2].to_vec())?;
                        buffer.clear(); 

                        match header_line.split_once(":") {
                            Some(v) => headers.insert(v.0.to_string(), v.1.to_string()),
                            None => return Err(Error::HeaderParsingError("In the loop".to_owned())),
                        };
                    }
                },
                Err(_) => break,
            }
        }

        let mut response_info_split = response_info.split(" ");

        let version = match response_info_split.next() {
            Some(v) => v.to_string(),
            None => return Err(Error::HeaderParsingError("Version".to_owned()))
        };

        let status_code = match response_info_split.next() {
            Some(v) => v.parse().unwrap(),
            None => return Err(Error::HeaderParsingError("Status code".to_owned()))
        };

        let status_name = match response_info_split.next() {
            Some(v) => v.to_string(),
            None => return Err(Error::HeaderParsingError("Status name".to_owned()))
        };

        if buffer.len() > 0 {
            body = Some(buffer);
        }

        Ok(Response {
            version,
            status_code,
            status_name,
            headers,
            body,
        })
    }
}