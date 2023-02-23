use tokio::{net::TcpStream, io::{AsyncWriteExt}, fs::OpenOptions};

use std::{collections::HashMap, path::PathBuf, time::{SystemTime}, sync::mpsc::channel};

use super::{
    url_parser::UrlParser, 
    request::Request,
    response::Response, 
    methods::Method
};

use crate::error::Error;

pub struct Connection {
    pub parsed_url: UrlParser,
}

impl Connection {
    pub async fn new(url: &str) -> Result<Self, Error> {
        let parsed_url = UrlParser::from(url).unwrap();

        Ok(Connection { parsed_url })
    }

    pub async fn request(&mut self,request: Request) -> Result<Response, Error> {
        let mut stream = TcpStream::connect(
            format!("{}:{}", &self.parsed_url.hostname, &self.parsed_url.port)
        ).await?;

        let _ = stream.write_all(format!("{} {} HTTP/1.1\r\n", request.get_method(), &self.parsed_url.path).as_bytes()).await?;
        let _ = stream.write_all(format!("HOST: {}\r\n", &self.parsed_url.hostname).as_bytes()).await?;
        for header in request.get_headers() {
            let _ = stream.write_all(
                format!("{}: {}\r\n", header.0, header.1).as_bytes()
            ).await?;
        }
        if let Some(range) = request.get_range() {
            let _ = stream.write_all(
                format!("Range: bytes={}-{}\r\n", range.start, range.end).as_bytes()
            ).await?;
        }
        let _ = stream.write_all(b"Connection: Close\r\n").await?;
        let _ = stream.write_all(b"\r\n\r\n").await?;

        Ok(Response::new(&mut stream).await?)
    } 

    pub async fn download(&mut self, path: PathBuf) -> Result<(), Error> {
        let head_request = Request::new().set_method(Method::HEAD);
        let head_request_response = self.request(head_request).await?;
        println!("{:#?}", head_request_response.headers);

        let mut file_path = path;

        let mut file_name = String::new();

        if file_path.is_dir() {
            if let Some(fname) = &self.parsed_url.file {
                file_name.push_str(fname);
            } else {
                if let Some(content_disposition) = head_request_response.headers.get("Content-Disposition") {
                    let split = content_disposition.split("=").last().unwrap();
                    file_name.push_str(split.trim_matches('"'));
                } else {
                    // generate file name
                    let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                        Ok(n) => n.as_secs(),
                        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
                    };
                    file_name.push_str(&format!("Download-{}", now));
                }
            }

            file_path = file_path.join(file_name);
        }

        let mut file = OpenOptions::new().create(true).read(true).write(true).open(&file_path).await?;

        let content_length = match head_request_response.headers.get("Content-Length") {
            Some(v) => v.trim().parse().unwrap(),
            None => 0
        };

        let connection_count = 5;
        let each_segment = 2_000_000;

        if content_length != 0 && content_length > each_segment {
            /* let (tx, rx) = channel(); */
        }

        let get_request = Request::new();

        let get_request_response = self.request(get_request).await?;

        file.write_all(get_request_response.body.unwrap().as_slice()).await?;

        println!("File downloaded: {}", file_path.to_str().unwrap());

        Ok(())
    }
}