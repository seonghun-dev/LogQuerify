mod store;

pub struct LocalFileStorage {
    pub host: String,
    pub path: String,
}

impl LocalFileStorage {
    pub fn new(host: String, path: String) -> Self {
        Self { host, path }
    }
}
