use std::{ fmt::Display, fs::{ DirEntry, FileType}, os::unix::fs::MetadataExt};

pub struct Entry {
    pub name: String,
    pub icon: String,
    pub size: u64,
    pub group_user: String,
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
    return Entry {
        name: dir_entry.file_name().to_str().unwrap().to_string().into(),
        icon: get_icon(dir_entry.file_type().expect("Could not get a file type")),
        size: get_entry_size(&dir_entry), 
        group_user: get_enrty_group_owner(&dir_entry),
        permissions: "1223".to_string()
    }

}

fn get_entry_size(dir_entry: &DirEntry) -> u64 {
    if let Ok(metadata) = dir_entry.metadata() {
        return metadata.len();
    }
    return 0;
}

fn get_entry_group_user(dir_entry: &DirEntry) -> String {
    if let Ok(metadata) = dir_entry.metadata() {
        let user = metadata.uid().to_string();
        let group = metadata.gid().to_string();

        return group + user;
    }

    return "".to_string();

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
