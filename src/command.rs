use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub command: String,
    pub envs: HashMap<String, String>,
    pub args: Vec<String>,
    pub metadata: HashMap<String, String>,
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

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        match serde_json::from_str(value) {
            Ok(command) => command,
            Err(e) => {
                eprintln!("[ERROR] {e}");
                Command::default()
            }
        }
    }
}

impl Default for Command {
    fn default() -> Self {
        Self {
            command: "/bin/nologin".into(),
            envs: HashMap::default(),
            args: Vec::default(),
            metadata: HashMap::default(),
        }
    }
}