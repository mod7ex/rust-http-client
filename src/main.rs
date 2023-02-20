mod error;
mod client;

use client::connection::Connection;
use error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<Error>>  {
    let mut connection = Connection::new("http://azbuka-novostroek.com").await?;

    connection.head_request().await?;

    Ok(())
}
