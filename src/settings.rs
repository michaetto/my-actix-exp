use std::env;
use config::{ConfigError, Config, File, Environment};

#[derive(Debug, Deserialize)]
struct Http {
    protocol: &str,
    host: &str,
    port: &str,
}

#[derive(Debug, Deserialize)]
struct RelationalDatabase {
    url: &str,
}

#[derive(Debug, Deserialize)]
struct DocumentDatabase {
    url: &str,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    debug: bool,
    r_db: RelationalDatabase,
    doc_db: DocumentDatabase
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.try_into()
    }
}
