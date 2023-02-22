use tokio::{net::TcpStream, io::{AsyncWriteExt}};

use std::collections::HashMap;

use super::url_parser::UrlParser;
use super::response::Response;
use super::methods::Method;

use crate::error::Error;

pub struct Connection {
    pub parsed_url: UrlParser,
}

impl Connection {
    pub async fn new(url: &str) -> Result<Self, Error> {
        let parsed_url = UrlParser::from(url).unwrap();

        Ok(Connection { parsed_url })
    }

    pub async fn request(
        &mut self,
        method: Method,
        request_headers: Option<HashMap<String, String>>
    ) -> Result<Response, Error> {
        let mut stream = TcpStream::connect(
            format!("{}:{}", &self.parsed_url.hostname, &self.parsed_url.port)
        ).await?;

        let _ = stream.write_all(format!("{} {} HTTP/1.1\r\n", method, &self.parsed_url.path).as_bytes()).await?;
        let _ = stream.write_all(format!("HOST: {}\r\n", &self.parsed_url.hostname).as_bytes()).await?;
        if let Some(headers) = request_headers {
            for header in headers {
                let _ = stream.write_all(
                    format!("{}: {}\r\n", header.0, header.1).as_bytes()
                ).await?;
            }
        }
        let _ = stream.write_all(b"Connection: Close\r\n").await?;
        let _ = stream.write_all(b"\r\n\r\n").await?;

        Ok(Response::new(&mut stream).await?)
    } 
}