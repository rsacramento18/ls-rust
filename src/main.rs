use std::{
    env,
    path::{Path, PathBuf},
};

mod entries;

fn get_path_string() -> PathBuf {
    if let Some(arg) = env::args().nth(1) {
        let mut path = PathBuf::new();
        path.push(arg);
        return path;
    } else {
        match env::current_dir() {
            Ok(curr) => return curr,
            Err(e) => panic!("Could not get current dir"),
        }
    }
}

fn main() {

    let path = get_path_string();

    path.read_dir().expect("Could not read directory");
        .map(|entry| Entry)
        .for_each(|entry| println!("{:?}", entry));
    println!("Hello, world!");
}
