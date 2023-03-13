use crate::{config::Config, traits::YamlFileSync};
use std::path::Path;

#[test]
fn test_config_load() {
    let path = Path::new("config.yaml");
    let mut config = Config::new();
    config.load(path);

    assert_eq!(config.sessions_dir(), "sessions")
}

#[test]
fn test_config_save() {
    let path = Path::new("config.yaml");
    let mut config = Config::new();
    config.load(path);
    let prv_default_session = config.default_session().clone();
    config.save(path);

    let mut config = Config::new();
    config.load(path);
    assert_eq!(
        prv_default_session.as_str(),
        config.default_session().as_str()
    );
}
