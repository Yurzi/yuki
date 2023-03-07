use std::{fs::OpenOptions, io::Read, path::Path};

fn main() {
    let path = Path::new("config.yaml");
    let file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(path);
    let mut file = match file {
        Err(why) => panic!("can't open {}: {:?}", path.display(), why),
        Ok(file) => file,
    };

    let mut buf = String::new();
    if let Err(why) = file.read_to_string(&mut buf) {
        panic!("can't read {}: {:?}", path.display(), why)
    }

    println!("{buf}");
}
