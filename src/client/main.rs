#![cfg_attr(not(unix), allow(unused_imports))]

pub mod client;

use libcommand::internal::{AuthorizeRequest};

#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = client::connect().await?;

    let request = tonic::Request::new(AuthorizeRequest {
        identifier: "Tonic".into(),
        public_ssh_keys: "Tonic".into(),
    });

    let response = client.authorize(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

#[cfg(not(unix))]
fn main() {
    panic!("The `uds` example only works on unix systems!");
}