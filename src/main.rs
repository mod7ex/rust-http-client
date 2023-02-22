mod error;
mod client;

use client::{ connection::Connection, methods::Method };
use error::Error;

#[tokio::main]
async fn main() -> Result<(), Error>  {
    let mut connection = Connection::new("http://site.azbuka-novostroek.com").await?;

    let response = connection.request(Method::GET, None).await?;

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
