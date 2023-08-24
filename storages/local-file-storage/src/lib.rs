use {
    gluesql::core::data::Schema,
    std::path::{Path, PathBuf},
};

mod store;

pub struct LocalFileStorage {
    pub host: String,
    pub path: PathBuf,
    pub schema: Schema,
}

impl LocalFileStorage {
    pub fn new(host: String, path: PathBuf, schema: Schema) -> Self {
        Self { host, path, schema }
    }
}
