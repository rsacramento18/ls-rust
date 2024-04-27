use std::{
    fmt::Display,
    fs::{DirEntry, FileType},
    os::unix::fs::{MetadataExt, PermissionsExt},
};

use human_bytes::human_bytes;
use users::{get_group_by_gid, get_user_by_uid};
use colored::Colorize;

pub struct Entry {
    pub name: String,
    pub extention: String,
    pub size: String,
    pub group_user: String,
    pub permissions: String,
    pub file_type: usize,
}

fn get_extention_icon(dir_entry: &DirEntry) -> String {
    if let Some(ext) = dir_entry.path().extension() {
        match ext.to_str().unwrap() {
            "lock" => return "".to_string(),
            "toml" => return "".to_string(),
            _ => return ext.to_str().unwrap().to_string(),
        }
    }

    match dir_entry.file_name().to_str().unwrap() {
        ".gitignore" => return "".to_string(),
        _ => return "".to_string(),

    }
}

fn get_type(file_type: FileType) -> usize {
    if file_type.is_dir() {
        return 0;
    } else if file_type.is_file() {
        return 1;
    } else {
        return 2;
    }
}

pub fn dir_entry_to_entry(dir_entry: DirEntry) -> Entry {
    return Entry {
        name: dir_entry.file_name().to_str().unwrap().to_string().into(),
        extention: get_extention_icon(&dir_entry),
        size: get_entry_size(&dir_entry),
        group_user: get_entry_group_user(&dir_entry),
        permissions: get_entry_permissions(&dir_entry),
        file_type: get_type(dir_entry.file_type().unwrap()),
    };
}

fn get_entry_size(dir_entry: &DirEntry) -> String {
    if let Ok(metadata) = dir_entry.metadata() {
        return human_bytes(metadata.len() as f64);
    }
    return "".to_string();
}

fn get_entry_group_user(dir_entry: &DirEntry) -> String {
    if let Ok(metadata) = dir_entry.metadata() {
        let uid = metadata.uid();
        let gid = metadata.gid();

        let user = get_user_by_uid(uid).unwrap();
        let group = get_group_by_gid(gid).unwrap();

        return format!(
            "{} {}",
            group.name().to_str().unwrap(),
            user.name().to_str().unwrap()
        );
    }

    return "".to_string();
}

fn get_entry_permissions(dir_entry: &DirEntry) -> String {
    if let Ok(metadata) = dir_entry.metadata() {
        print!("{}", metadata.permissions().mode());
        
    }
    return "".to_string();

}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formated = format!(
            "{} {} {} {} {}",
            self.permissions, self.group_user, self.size, self.name, self.extention
        );

        match self.file_type {
            0 => write!(f, "{}", formated.blue()),
            1 => write!(f, "{}", formated),
            2 => write!(f, "{}", formated.green()),
            _ => write!(f, "{}", formated),
        }
    }
}
