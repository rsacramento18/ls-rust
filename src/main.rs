use std::{env, path::{Path, PathBuf}};

fn get_path_string() -> PathBuf{
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
    println!("Hello, world!");
}
