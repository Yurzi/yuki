#![allow(dead_code)]
#![allow(unused_variables)]

use crate::traits::YamlFileSync;
use serde::{Deserialize, Serialize};
use std::default::Default;

// the struct to present the config.yaml
#[derive(Serialize, Deserialize)]
pub struct Config {
    api_key: String,

    #[serde(default)]
    sessions_dir: String,

    #[serde(default)]
    roles_dir: String,

    default_session: String,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Config {
    pub fn api_key(&self) -> &String {
        &self.api_key
    }

    pub fn sessions_dir(&self) -> &String {
        &self.sessions_dir
    }

    pub fn roles_dir(&self) -> &String {
        &self.roles_dir
    }

    pub fn default_session(&self) -> &String {
        &self.default_session
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api_key: "".to_string(),
            sessions_dir: "sessions".to_string(),
            roles_dir: "roles".to_string(),
            default_session: "".to_string(),
        }
    }
}

impl YamlFileSync for Config {}
