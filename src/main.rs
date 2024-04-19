use std::{env, path::PathBuf};

use crate::entries::entry::dir_entry_to_entry;

mod entries;

fn get_path_string() -> PathBuf {
    if let Some(arg) = env::args().nth(1) {
        let mut path = PathBuf::new();
        path.push(arg);
        return path;
    } else {
        match env::current_dir() {
            Ok(curr) => return curr,
            Err(e) => panic!("Could not get current dir - {}", e),
        }
    }
}

fn main() -> std::io::Result<()> {
    let path = get_path_string();

    path.read_dir()
        .expect("Could not read diran use my_string.into() to create an OsStrinectory")
        .map(|entry| return dir_entry_to_entry(entry.unwrap()))
        .for_each(|entry| println!("{}", entry));
    Ok(())
}
