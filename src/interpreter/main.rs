#![cfg_attr(not(unix), allow(unused_imports))]

pub mod client;

use libcommand::interpreter::{AuthorizeRequest, AuthorizeResponse, TerminateRequest};
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
        command: command_arg.command.clone(),
        pid: std::process::id()
    });

    let response : Response<AuthorizeResponse> = client.authorize(request).await?;

    let mut command : std::process::Command = command_arg.into();
    let mut child = command.spawn().unwrap();
    child.wait().unwrap();

    client.terminate(tonic::Request::new(TerminateRequest {
        session_id: response.get_ref().session_id.clone(),
        log_file: "".to_string()
    })).await?;

    Ok(())
}

#[cfg(not(unix))]
fn main() {
    panic!("The `uds` example only works on unix systems!");
}