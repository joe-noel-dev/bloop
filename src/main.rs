mod api;
mod database;
mod generators;
mod model;
mod network;

extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    network::manager::run().await;
    Ok(())
}
