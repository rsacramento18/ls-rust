use std::{ ffi::OsString, fmt::Display, fs::DirEntry};

pub struct Entry {
    pub name: OsString,
    pub icon: String,
    pub size: usize,
    pub group: String,
    pub owner: String,
    pub permissions: String,
}

pub fn dir_entry_to_entry(dir_entry: DirEntry) -> Entry{
    return Entry {
        name: dir_entry.file_name(),
        icon: "folder".to_string(),
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
            "{} {} {} {} {:?} {}",
            self.permissions,
            self.group,
            self.owner,
            self.size,
            self.name,
            self.icon
        )
    }

}
