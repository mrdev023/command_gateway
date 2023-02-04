mod whitelist;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Configuration {
    whitelist: whitelist::Whitelist,
}

impl Configuration {
    pub fn command_allowed(&self, command: &str) -> bool {
        self.whitelist == command
    }
}