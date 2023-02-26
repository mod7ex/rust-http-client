#![allow(unused)]

mod error;
mod client;

use std::path::PathBuf;
use client::{connection::Connection, request::Request, methods::Method};
use error::Error;
use tokio::{fs::OpenOptions, io::AsyncReadExt};

/*

#[tokio::main]
async fn main() -> Result<(), Error>  {
    let mut connection = Connection::new("http://crm.metriks.ru/upload/resize_cache/itiso.shahmatki/3a3/460_260_1/c6x2bymyf2h971l17v70m17pmt8b2p5o.jpg").await?;

    let response = connection.request(Method::GET, None).await?;

    println!("{:#?}", connection.parsed_url);

    let body_content = match response.body {
        Some(v) => {
            String::from_utf8(v).expect("Found invalid UTF-8")
        },
        None => String::new(),
    };

    println!("{:#?}", body_content);

    Ok(())
}

*/

/*

#[tokio::main]
async fn main() -> Result<(), Error>  {
    let mut connection = Connection::new("http://127.0.0.1:3000/file").await?;

    connection.download(PathBuf::from("/home/mourad/Desktop")).await?;

    println!("{:#?}", connection.parsed_url);

    Ok(())
}

*/

/*

#[tokio::main]
async fn main() -> Result<(), Error>  {
    if let Ok(connection) = Connection::new("http://127.0.0.1:3000").await {
        let request = Request::new()
            .add_query("name", "Mourad")
            .add_query("age", "25");

        println!("{:#?}", request);
        let response = connection.request(request).await?;
        println!("{:#?}", response);
    }

    Ok(())
}
*/

/*

#[tokio::main]
async fn main() -> Result<(), Error>  {
    if let Ok(connection) = Connection::new("http://127.0.0.1:3000/urlencoded").await {
        let request = Request::new()
            .set_method(Method::POST)
            .form_data()
            .add_form_data("name", "Mourad")
            .add_form_data("age", "25");

        println!("{:#?}", request);
        let b = request.get_body().unwrap().to_owned();
        println!("{}", String::from_utf8(b).unwrap());

        let response = connection.request(request).await?;
        println!("{:#?}", response);
    }

    Ok(())
}
*/

/**/

#[tokio::main]
async fn main() -> Result<(), Error>  {
    if let Ok(connection) = Connection::new("http://127.0.0.1:3000/multipart-form-data").await {
        let file_path: PathBuf = "/home/mourad/Desktop/rust/http-client/file.txt".into();
        let mut file = OpenOptions::new()
            .read(true)
            .open(file_path)
            .await?;
        let mut buffer = vec![];
        file.read_to_end(&mut buffer).await?;
        
        let request = Request::new()
            .set_method(Method::POST)
            .multipart()
            .add_data("name", "Mourad")
            .add_data("age", "25")
            .add_file("file", "file.txt", buffer)
            .close_multipart_form_data();

        println!("{:#?}", request);
        let b = request.get_body().unwrap().to_owned();
        println!("{}", String::from_utf8(b).unwrap());

        let response = connection.request(request).await?;
        println!("{:#?}", response);
    }

    Ok(())
}