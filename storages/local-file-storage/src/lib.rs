use {
    async_trait::async_trait,
    gluesql::{
        core::{
            data::Schema,
            store::{
                AlterTable, CustomFunction, CustomFunctionMut, DataRow, Index, IndexMut, Metadata,
                StoreMut, Transaction,
            },
        },
        prelude::{Error, Key, Result},
    },
    std::path::PathBuf,
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

impl CustomFunction for LocalFileStorage {}
impl CustomFunctionMut for LocalFileStorage {}
impl Index for LocalFileStorage {}
impl IndexMut for LocalFileStorage {}
impl Metadata for LocalFileStorage {}
impl AlterTable for LocalFileStorage {}
impl Transaction for LocalFileStorage {}

#[async_trait(?Send)]
impl StoreMut for LocalFileStorage {
    async fn insert_schema(&mut self, _schema: &Schema) -> Result<()> {
        let msg = "[Storage] StoreMut::insert_schema is not supported".to_owned();
        Err(Error::StorageMsg(msg))
    }

    async fn delete_schema(&mut self, _table_name: &str) -> Result<()> {
        let msg = "[Storage] StoreMut::delete_schema is not supported".to_owned();
        Err(Error::StorageMsg(msg))
    }

    async fn append_data(&mut self, _table_name: &str, _rows: Vec<DataRow>) -> Result<()> {
        let msg = "[Storage] StoreMut::append_data is not supported".to_owned();
        Err(Error::StorageMsg(msg))
    }

    async fn insert_data(&mut self, _table_name: &str, _rows: Vec<(Key, DataRow)>) -> Result<()> {
        let msg = "[Storage] StoreMut::insert_data is not supported".to_owned();
        Err(Error::StorageMsg(msg))
    }

    async fn delete_data(&mut self, _table_name: &str, _keys: Vec<Key>) -> Result<()> {
        let msg = "[Storage] StoreMut::delete_data is not supported".to_owned();
        Err(Error::StorageMsg(msg))
    }
}
