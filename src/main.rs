use std::{env, path::{Path, PathBuf}};

fn get_path_string() -> PathBuf{
    if let Some(arg) = env::args().nth(1) {
        let path = PathBuf::new();
        path.pushErr(())
        return path;
    } else {
        match env::current_dir() {
            Ok(curr) => return curr,
            Err => println!("Error"),
        }
    }
    Err(())
}

fn main() {
    println!("Hello, world!");
}
