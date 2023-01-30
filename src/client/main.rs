#![cfg_attr(not(unix), allow(unused_imports))]

pub mod internal {
    tonic::include_proto!("internal");
}

use internal::{unix_client::UnixClient, AuthorizeRequest};
#[cfg(unix)]
use tokio::net::UnixStream;
use tonic::transport::{Endpoint, Uri};
use tower::service_fn;

#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Endpoint::try_from("http://[::]:50051")?
        .connect_with_connector(service_fn(|_: Uri| {
            // Connect to a Uds socket
            UnixStream::connect(libcommand::SOCK_FILE)
        }))
        .await?;

    let mut client = UnixClient::new(channel);

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