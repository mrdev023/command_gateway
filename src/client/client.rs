use libcommand::internal::unix_client::UnixClient;

#[cfg(unix)]
use tokio::net::UnixStream;
use tonic::transport::{Channel, Endpoint, Uri};
use tower::service_fn;

pub(super) async fn connect() -> Result<UnixClient<Channel>, Box<dyn std::error::Error>> {
    let channel = Endpoint::try_from("http://[::]:50051")?
        .connect_with_connector(service_fn(|_: Uri| UnixStream::connect(libcommand::SOCK_FILE)))
        .await?;

    Ok(UnixClient::new(channel))
}
