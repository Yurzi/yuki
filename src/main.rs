mod config;

use config::Config;
use std::path::Path;

fn main() {
    let path = Path::new("config.yaml");
    let config = Config::load(path);

    println!("api_key: {}", config.api_key());
}
