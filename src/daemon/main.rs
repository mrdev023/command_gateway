#![cfg_attr(not(unix), allow(unused_imports))]

use std::path::Path;
#[cfg(unix)]
use tokio::net::UnixListener;
#[cfg(unix)]
use tokio_stream::wrappers::UnixListenerStream;
#[cfg(unix)]
use tonic::transport::server::UdsConnectInfo;
use tonic::{transport::Server, Request, Response, Status};

pub mod internal {
    tonic::include_proto!("internal");
}

use internal::{
    unix_server::{Unix, UnixServer},
    AuthorizeRequest, AuthorizeResponse, TerminateRequest, TerminateResponse
};

#[derive(Default)]
pub struct DaemonServer {}

#[tonic::async_trait]
impl Unix for DaemonServer {
    async fn authorize(
        &self,
        request: Request<AuthorizeRequest>,
    ) -> Result<Response<AuthorizeResponse>, Status> {
        #[cfg(unix)]
        {
            let conn_info = request.extensions().get::<UdsConnectInfo>().unwrap();
            println!("Got a request {:?} with info {:?}", request, conn_info);
        }

        let reply = internal::AuthorizeResponse {
            status: internal::AuthorizationStatus::Authorized.into(),
            error_message: "".into(),
            log_file: "".into(),
            session_uuid: "".into()
        };
        Ok(Response::new(reply))
    }

    async fn terminate(
        &self,
        request: Request<TerminateRequest>,
    ) -> Result<Response<TerminateResponse>, Status> {
        #[cfg(unix)]
        {
            let conn_info = request.extensions().get::<UdsConnectInfo>().unwrap();
            println!("Got a request {:?} with info {:?}", request, conn_info);
        }

        let reply = internal::TerminateResponse {
            status: internal::TerminateStatus::Ok.into(),
            error_message: "".into(),
        };
        Ok(Response::new(reply))
    }
}

#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(Path::new(libcommand::SOCK_FILE).parent().unwrap())?;

    let server = DaemonServer::default();

    let uds = UnixListener::bind(libcommand::SOCK_FILE)?;
    let uds_stream = UnixListenerStream::new(uds);

    Server::builder()
        .add_service(UnixServer::new(server))
        .serve_with_incoming(uds_stream)
        .await?;

    Ok(())
}

#[cfg(not(unix))]
fn main() {
    panic!("The `uds` example only works on unix systems!");
}