use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub const SOCK_FILE : &'static str = "command_gateway.sock";

pub mod internal {
    tonic::include_proto!("internal");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    command: String,
    envs: HashMap<String, String>,
    args: Vec<String>
}

impl Into<std::process::Command> for Command {
    fn into(self) -> std::process::Command {
        let mut command = std::process::Command::new("/usr/bin/script");
        command.env_clear();
        let final_command = format!("{} {}", self.command, self.args.join(" "));
        command.args(vec!["-q", "-f", "session.log", "-c", &final_command]);
        command.envs(self.envs);
        command
    }
}