/*
 *  Some Traits Use by this project
 *
 */

use serde::{de::DeserializeOwned, Serialize};
use std::{
    default::Default,
    fs::File,
    io::{Read, Write},
    path::Path,
};
pub trait Vaildate {
    fn is_vaild(&self) -> Result<bool, String>;
    fn make_vaild(&mut self) -> Result<bool, String>;
}

pub trait YamlFileSync: Default + Serialize + DeserializeOwned {
    fn load(&mut self, path: &Path) {
        // create default file when not exist
        match path.try_exists() {
            Ok(false) => Self::default().save(path),
            Err(_) => panic!("permission denied during check {}", path.display()),
            _ => {}
        }
        let mut fd =
            File::open(path).unwrap_or_else(|_| panic!("{} can't be opened", path.display()));

        let mut yaml_str: String = String::new();
        fd.read_to_string(&mut yaml_str).expect("internal error!");

        let res: Self = serde_yaml::from_str(&yaml_str)
            .unwrap_or_else(|_| panic!("{} has syntax errors!", path.display()));

        *self = res;
    }

    fn save(self, path: &Path) {
        let yaml_str = serde_yaml::to_string(&self).unwrap_or_else(|_| panic!("bad syntax"));

        let mut fd =
            File::create(path).unwrap_or_else(|_| panic!("{} can't be opened", path.display()));

        fd.write_all(yaml_str.as_bytes()).unwrap();
    }
}
