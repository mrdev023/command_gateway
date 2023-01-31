use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub identifier: String,
    pub token: String,
    pub command: String,
    pub envs: HashMap<String, String>,
    pub args: Vec<String>
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