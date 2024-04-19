use std::{fmt::Display, fs::{DirEntry, FileType}};

pub struct Entry {
    pub name: String,
    pub icon: String,
    pub size: usize,
    pub group: String,
    pub owner: String,
    pub permissions: String,
}


fn get_icon(fileType: FileType) -> String {
    if fileType.is_dir() {
        return "".to_string();
    } else if fileType.is_file() {
        return "".to_string();
    } else {
        return "󱀶".to_string();
    }

}


pub fn dir_entry_to_entry(dir_entry: DirEntry) -> Entry{
    println!("{:?}", dir_entry.metadata());
    return Entry {
        name: dir_entry.file_name().to_str().unwrap().to_string().into(),
        icon: get_icon(dir_entry.file_type().expect("Could not get a file type")),
        size: 13,
        group: "rsacramento".to_string(),
        owner: "rsacramento".to_string(),
        permissions: "1223".to_string()
    }

}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "{} {} {} {} {} {}",
            self.permissions,
            self.group,
            self.owner,
            self.size,
            self.name,
            self.icon
        )
    }
}
