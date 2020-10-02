mod api;
mod generators;
mod model;
mod server;

extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    server::server::run().await;
    Ok(())
}
