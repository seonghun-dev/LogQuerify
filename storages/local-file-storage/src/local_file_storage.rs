use gluesql::core::store::RowIter;

use {
    gluesql::{
        core::{ast::ColumnDef, data::Schema},
        prelude::Result,
    },
    std::{
        fs::File,
        io::{self, BufRead},
        path::{Path, PathBuf},
    },
};

pub struct LocalFileStorage {
    pub host: String,
    pub path: PathBuf,
    pub column_defs: Vec<ColumnDef>,
}

impl LocalFileStorage {
    /*
     * Create a new LocalFileStorage instance.
     */
    pub fn new(host: String, path: PathBuf) -> Self {
        match path.exists() {
            true => {}
            false => {
                let msg = "[Storage] Can't find Path".to_owned();
                panic!("{}", msg);
            }
        }
        Self {
            host,
            path,
            column_defs: vec![],
        }
    }

    /*
     * Read all lines from the file.
     */
    fn path_by(&self, table_name: &str) -> PathBuf {
        let path = self.path.as_path();
        let mut path = path.join(table_name);
        path.set_extension("log");

        path
    }

    /*
     * Read all lines from the file.
     */
    pub fn fetch_schema(&self, table_name: &str) -> Result<Option<Schema>> {
        match self.path_by(table_name).exists() {
            true => {}
            false => {
                return Ok(None);
            }
        }
        let schema = Schema {
            table_name: table_name.to_owned(),
            column_defs: Some(self.column_defs.clone()),
            indexes: vec![],
            engine: None,
        };
        Ok(Some(schema))
    }

    fn scan_data(&self, table_name: &str) -> Result<(RowIter, Schema)> {
        todo!()
    }

    /*
     * Read all lines from the file.
     */
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}
