pub struct Base {
    pub title: String,
    pub status: String,
    pub last_update: String,
}

impl Base {
    pub fn new(title: &str, status: &str, last_update: &str) -> Self {
        Self {
            title: title.to_string(),
            status: status.to_string(),
            last_update: last_update.to_string(),
        }
    }
}
