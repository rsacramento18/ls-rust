use std::{fmt::Display, fs::DirEntry};

pub struct Entry {
    pub name: String,
    pub icon: String,
    pub size: usize,
    pub group: String,
    pub owner: String,
    pub permissions: String,
}

fn DirEntryToEntry(dirEntry: DirEntry) -> Entry{
    return Entry {
        name: dirEntry.file_name(),
        icon: "folder",
        size: "13",
        group: "rsacramento",
        owner: "rsacramento",
        permissions: "1223"
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
