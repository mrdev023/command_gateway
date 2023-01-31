#![cfg_attr(not(unix), allow(unused_imports))]

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
        _request: Request<AuthorizeRequest>,
    ) -> Result<Response<AuthorizeResponse>, Status> {
        let reply = AuthorizeResponse {
            status: AuthorizationStatus::Authorized.into(),
            session_uuid: uuid::Uuid::new_v4().to_string()
        };
        Ok(Response::new(reply))
    }

    async fn terminate(
        &self,
        _request: Request<TerminateRequest>,
    ) -> Result<Response<TerminateResponse>, Status> {
        let reply = TerminateResponse {
            status: TerminateStatus::Ok.into()
        };
        Ok(Response::new(reply))
    }
}