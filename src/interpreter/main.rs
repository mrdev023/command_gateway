#![cfg_attr(not(unix), allow(unused_imports))]

pub mod client;

use libcommand::interpreter::{AuthorizationStatus, AuthorizeRequest, AuthorizeResponse};
use tonic::Response;

#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = std::env::args()
        .skip(1)
        .last().unwrap();
    let command_arg : libcommand::Command = serde_json::from_str::<libcommand::Command>(&arg)
        .unwrap();

    let mut client = client::connect().await?;

    let request = tonic::Request::new(AuthorizeRequest {
        identifier: command_arg.identifier.clone(),
        token: command_arg.token.clone(),
        command: command_arg.command.clone()
    });

    let response : Response<AuthorizeResponse> = client.authorize(request).await?;

    if AuthorizationStatus::from_i32(response.get_ref().status) == Some(AuthorizationStatus::Authorized) {
        let mut command : std::process::Command = command_arg.into();
        let mut child = command.spawn().unwrap();
        child.wait().unwrap();
    } else {
        eprintln!("Permission denied");
    }

    Ok(())
}

#[cfg(not(unix))]
fn main() {
    panic!("The `uds` example only works on unix systems!");
}