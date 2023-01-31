mod command;
mod session;

pub use command::Command;

pub const SOCK_FILE : &'static str = "/var/run/command_gateway.sock";

pub mod interpreter {
    tonic::include_proto!("interpreter");
}

