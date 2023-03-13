#![cfg_attr(not(unix), allow(unused_imports))]

use tonic::{Code, Request, Response, Status};
use libcommand::Command;

use libcommand::interpreter::{
    unix_server::Unix,
    AuthorizeRequest, AuthorizeResponse,
    TerminateRequest, TerminateResponse
};

#[derive(Default)]
pub struct DaemonServer;

#[tonic::async_trait]
impl Unix for DaemonServer {
    async fn authorize(
        &self,
        request: Request<AuthorizeRequest>,
    ) -> Result<Response<AuthorizeResponse>, Status> {
        let session = libcommand::Session::from(request.get_ref().pid);
        let cmd = Command::from(request.get_ref().command_arg.as_ref());

        let conf = super::CONFIGURATION.lock()
            .map_err(|e| Status::internal(e.to_string()))?;
        let conf = conf.as_ref().ok_or_else(|| Status::internal("Configuration not loaded"))?;

        if !conf.command_allowed(&cmd.command) {
            return Err(Status::permission_denied("Command not authorized"));
        }

        let session_id = session.id.clone();
        super::SESSIONS.lock().unwrap().push(session);

        Ok(Response::new(AuthorizeResponse {
            command_arg: request.get_ref().command_arg.clone(),
            session_id
        }))
    }

    async fn terminate(
        &self,
        request: Request<TerminateRequest>,
    ) -> Result<Response<TerminateResponse>, Status> {
        let mut lock = super::get_sessions_lock()
            .map_err(|e| Status::new(Code::Internal, e))?;
        super::remove_session_by_id(&mut lock, &request.get_ref().session_id)
            .map_err(|e| Status::new(Code::NotFound, e))?;
        Ok(Response::new(TerminateResponse {}))
    }
}