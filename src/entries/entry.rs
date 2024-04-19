struct Entry {
    name: String,
    icon: String,
    size: usize,
    group: String,
    owner: String,
    permissions: String,
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
