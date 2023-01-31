mod command;
mod session;

pub use command::Command;
pub use session::Session;

pub const SOCK_FILE : &'static str = "command_gateway.sock";

pub mod interpreter {
    tonic::include_proto!("interpreter");
}

