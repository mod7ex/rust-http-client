use tokio::{net::TcpStream, io::{AsyncWriteExt, AsyncReadExt}};

use super::url_parser::UrlParser;

use crate::error::Error;

pub struct Connection {
    pub parsed_url: UrlParser,
    pub stream: TcpStream,
}

impl Connection { 
    pub async fn new(url: &str) -> Result<Self, Error> {
        let parsed_url = UrlParser::from(url).unwrap();

        let addr = format!("{}:80", parsed_url.host);

        let stream = TcpStream::connect(&addr).await?;

        Ok(Connection { parsed_url, stream })
    }

    pub async fn head_request(&mut self) -> Result<(), Error> {
        let _ = &self.stream.write_all(format!("HEAD {} HTTP/1.1\r\n", &self.parsed_url.path).as_bytes()).await?;
        let _ = &self.stream.write_all(format!("HOST: {}\r\n", &self.parsed_url.host).as_bytes()).await?;
        let _ = &self.stream.write_all(b"Connection: Close\r\n").await?;
        let _ = &self.stream.write_all(b"\r\n\r\n").await?;

        let mut buffer = String::new();

        let _ = &self.stream.read_to_string(&mut buffer).await?;

        println!("Response\n {}", buffer);

        Ok(())
    }
}