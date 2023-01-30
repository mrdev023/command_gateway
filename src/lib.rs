use serde::{Serialize, Deserialize};

pub const SOCK_FILE : &'static str = "command_gateway.sock";

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {

}