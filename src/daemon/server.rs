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
        let cmd_arg = request.get_ref().command_arg.clone();
        let cmd = Command::from(cmd_arg.as_str());

        let endpoint  = {
            let conf_mutex = super::CONFIGURATION.lock()
                .map_err(|e| Status::internal(e.to_string()))?;
            let conf = conf_mutex.as_ref().ok_or_else(|| Status::internal("Configuration not loaded"))?;

            if !conf.command_allowed(&cmd.command) {
                return Err(Status::permission_denied("Command not authorized"));
            }

            conf.get_endpoint()
                .and_then(|endpoint| Some(endpoint.clone()))
        };

        if let Some(endpoint) = endpoint {
            let client = reqwest::Client::new();
            let res = client.post(endpoint)
                .body(cmd_arg.clone())
                .send()
                .await
                .map_err(|_| Status::internal("ENDPOINT: Not reachable"))?;
            let status = res.status();

            if status.is_client_error() {
                return match status {
                    reqwest::StatusCode::UNAUTHORIZED => {
                        Err(Status::unauthenticated("ENDPOINT: Not authorized"))
                    },
                    reqwest::StatusCode::BAD_REQUEST => {
                        Err(Status::unauthenticated("ENDPOINT: Invalid metadata"))
                    },
                    _ => {
                        Err(Status::permission_denied("ENDPOINT: Permission denied"))
                    }
                }
            } else if status.is_server_error() {
                return Err(Status::internal("ENDPOINT: Internal error"));
            } else if status.is_informational() || status.is_redirection() {
                return Err(Status::unimplemented("ENDPOINT: Status code not supported"));
            }
        }

        let session_id = session.id.clone();
        super::SESSIONS.lock().unwrap().push(session);

        Ok(Response::new(AuthorizeResponse {
            command_arg: cmd_arg,
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