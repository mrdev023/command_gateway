mod whitelist;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Configuration {
    whitelist: whitelist::Whitelist,
    endpoint: Option<String>
}

impl Configuration {
    pub fn read_or_create() -> Self {
        let path = std::path::Path::new("configuration.yml");
        let file = std::fs::File::open(path)
            .map_err(|_| "No such file configuration.yml".to_string())
            .unwrap();
        let buffer = std::io::BufReader::new(file);
        serde_yaml::from_reader(buffer).unwrap()
    }

    pub fn command_allowed(&self, command: &str) -> bool {
        self.whitelist == command
    }

    pub fn get_endpoint(&self) -> Option<&String> {
        self.endpoint.as_ref()
    }
}