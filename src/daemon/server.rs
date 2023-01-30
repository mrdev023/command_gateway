#![cfg_attr(not(unix), allow(unused_imports))]

#[cfg(unix)]
use tonic::transport::server::UdsConnectInfo;
use tonic::{transport::Server, Request, Response, Status};

use libcommand::internal::{
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

        let reply = libcommand::internal::AuthorizeResponse {
            status: libcommand::internal::AuthorizationStatus::Authorized.into(),
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

        let reply = libcommand::internal::TerminateResponse {
            status: libcommand::internal::TerminateStatus::Ok.into(),
            error_message: "".into(),
        };
        Ok(Response::new(reply))
    }
}