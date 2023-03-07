use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

// the struct to present the config.yaml
#[derive(Serialize, Deserialize)]
pub struct Config {
    api_key: String,
    sessions_dir: String,
    default_session: String,
}

impl Config {
    pub fn load(path: &Path) -> Config {
        let mut fd =
            File::open(path).unwrap_or_else(|_| panic!("{} can't be opend", path.display()));

        let mut yaml_str: String = String::new();
        fd.read_to_string(&mut yaml_str).expect("internal error!");

        let config: Config = serde_yaml::from_str(&yaml_str)
            .unwrap_or_else(|_| panic!("{} has syntax errors!", path.display()));

        // return this value
        config
    }

    pub fn save(self, path: &Path) {
        let yaml_str = serde_yaml::to_string(&self).unwrap_or_else(|_| panic!("bad syntax"));

        let mut fd =
            File::create(path).unwrap_or_else(|_| panic!("{} can't be opend", path.display()));

        fd.write_all(yaml_str.as_bytes()).unwrap();
    }
}

#[test]
fn test_config_load() {
    let path = Path::new("config.yaml");
    let config = Config::load(path);

    assert_eq!(config.sessions_dir, "sessions")
}

#[test]
fn test_config_save() {
    let path = Path::new("config.yaml");
    let mut config = Config::load(path);

    config.default_session = "qwq".to_string();
    let prv_default_session = config.default_session.clone();

    config.save(path);

    let res = Config::load(path);

    assert_eq!(prv_default_session, res.default_session);
}
