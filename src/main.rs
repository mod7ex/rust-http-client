mod error;
mod client;

use client::connection::Connection;
use error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<Error>>  {
    let mut connection = Connection::new("http://site.azbuka-novostroek.com:8000").await?;

    let response = connection.get_request().await?;

    /* println!("{:#?}", response); */

    let body_content = match response.body {
        Some(v) => {
            String::from_utf8(v).expect("Found invalid UTF-8")
        },
        None => String::new(),
    };

    println!("{:#?}", body_content);

    Ok(())
}
