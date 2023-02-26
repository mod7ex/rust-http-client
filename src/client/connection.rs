use tokio::{self, net::TcpStream, io::{AsyncWriteExt, AsyncSeekExt}, fs::OpenOptions, sync::Semaphore};

use std::{ path::PathBuf, time::{SystemTime}, sync::{mpsc::channel, Arc}};

use super::{
    url_parser::UrlParser, 
    request::Request,
    response::Response, 
    methods::Method
};

static SEM: Semaphore = Semaphore::const_new(0);

use crate::error::Error;

#[derive(Clone)]
pub struct Connection {
    pub parsed_url: UrlParser,
}

impl Connection {
    pub async fn new(url: &str) -> Result<Self, Error> {
        let parsed_url = UrlParser::from(url).unwrap();

        Ok(Connection { parsed_url })
    }

    pub async fn request(&self,request: Request) -> Result<Response, Error> {
        let mut stream = TcpStream::connect(
            format!("{}:{}", &self.parsed_url.hostname, &self.parsed_url.port)
        ).await?;

        let path = if request.get_query().is_empty() {
            self.parsed_url.path.to_owned()
        } else {
            format!("{}?{}", self.parsed_url.path, request.get_query())
        };

        let _ = stream.write_all(
            format!("{} {} HTTP/1.1\r\n", request.get_method(), path).as_bytes()
        ).await?;
        let _ = stream.write_all(
            format!("HOST: {}\r\n", &self.parsed_url.hostname).as_bytes()
        ).await?;
        for header in request.get_headers() {
            let _ = stream.write_all(
                format!("{}: {}\r\n", header.0, header.1).as_bytes()
            ).await?;
        }
        let _ = stream.write_all(
            format!("Content-Length: {}\r\n", request.get_content_length()).as_bytes()
        ).await?;
        if let Some(range) = request.get_range() {
            let _ = stream.write_all(
                format!("Range: bytes={}-{}\r\n", range.start, range.end).as_bytes()
            ).await?;
        }
        let _ = stream.write_all(b"Connection: Close\r\n").await?;

        if let Some(body_content) = request.get_body() {
            let _ = stream.write_all(body_content.as_slice()).await?;
        }

        let _ = stream.write_all(b"\r\n\r\n").await?;

        Ok(Response::new(&mut stream).await?)
    } 

    pub async fn download(&self, path: PathBuf) -> Result<(), Error> {
        let head_request = Request::new().set_method(Method::HEAD);
        let head_request_response = self.request(head_request).await?;

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
                        Err(_) => 0,
                    };
                    file_name.push_str(&format!("Download-{}", now));
                }
            }

            file_path = file_path.join(file_name);
        }

        let mut file = OpenOptions::new().create(true).read(true).write(true).open(&file_path).await?;

        // in Bytes 
        let content_length = match head_request_response.headers.get("Content-Length") {
            Some(v) => v.trim().parse().unwrap(),
            None => 0
        };

        SEM.add_permits(5);

        let each_segment = 500_000; // 5KB
        // check if server supports range request <Accept-Ranges: bytes> -- https://developer.mozilla.org/en-US/docs/Web/HTTP/Range_requests
        if content_length != 0 && content_length > each_segment {
            let (tx, rx) = channel();

            let mut left_steps = content_length / each_segment;

            let mut range = 0..each_segment;

            let arc_self = Arc::new(self.clone());

            tokio::spawn(async move {
                while let Ok(permit) = SEM.acquire().await {
                    let _self = arc_self.clone();
                    let current_range = range.clone();
                    let _tx = tx.clone();

                    tokio::spawn(async move {
                        println!("spawn");
                        let _permit = permit;
                        let request = Request::new().set_range(current_range.clone());
                        let mut response = _self.request(request).await.unwrap();

                        response.range = Some(current_range);

                        _tx.send(response).unwrap();
                    });

                    range = if range.end + each_segment > content_length {
                        range.end + 1..content_length
                    } else {
                        range.end + 1..range.end + each_segment
                    };

                    if left_steps == 0 {
                        break;
                    }
                    
                    left_steps -= 1;
                }
            });

            while let Ok(response) = rx.recv() {
                println!("received");
                file.seek(std::io::SeekFrom::Start(response.range.unwrap().start as u64)).await?;
                file.write_all(response.body.unwrap().as_slice()).await?;
            }
        } else {
            let get_request = Request::new();

            let get_request_response = self.request(get_request).await?;
    
            file.write_all(get_request_response.body.unwrap().as_slice()).await?;
        }

        println!("File downloaded: {}", file_path.to_str().unwrap());

        Ok(())
    }
}