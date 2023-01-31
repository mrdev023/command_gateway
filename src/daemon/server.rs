#![cfg_attr(not(unix), allow(unused_imports))]

#[cfg(unix)]
use tonic::transport::server::UdsConnectInfo;
use tonic::{Request, Response, Status};

use libcommand::internal::{
    unix_server::Unix,
    AuthorizeRequest, AuthorizeResponse, AuthorizationStatus,
    TerminateRequest, TerminateResponse, TerminateStatus
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

        let reply = AuthorizeResponse {
            status: AuthorizationStatus::Authorized.into(),
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

        let reply = TerminateResponse {
            status: TerminateStatus::Ok.into()
        };
        Ok(Response::new(reply))
    }
}