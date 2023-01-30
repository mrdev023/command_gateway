#![cfg_attr(not(unix), allow(unused_imports))]

pub mod client;

use libcommand::internal::{AuthorizeRequest};

#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let arg = std::env::args()
    //     .skip(1)
    //     .next().unwrap();
    let arg = String::from("{\"command\": \"nu\", \"envs\": {}, \"args\": []}");
    let mut command : std::process::Command = serde_json::from_str::<libcommand::Command>(&arg)
        .unwrap()
        .into();

    let mut client = client::connect().await?;

    let request = tonic::Request::new(AuthorizeRequest {
        identifier: "Tonic".into(),
        public_ssh_keys: "Tonic".into(),
    });

    let response = client.authorize(request).await?;

    println!("RESPONSE={:?}", response);

    let mut child = command.spawn().unwrap();
    child.wait().unwrap();

    Ok(())
}

#[cfg(not(unix))]
fn main() {
    panic!("The `uds` example only works on unix systems!");
}