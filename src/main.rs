mod error;
mod client;

use std::path::PathBuf;
use client::{connection::Connection};
use error::Error;

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

#[tokio::main]
async fn main() -> Result<(), Error>  {
    let mut connection = Connection::new("http://127.0.0.1:3000/file").await?;

    connection.download(PathBuf::from("/home/mourad/Desktop")).await?;

    println!("{:#?}", connection.parsed_url);

    Ok(())
}