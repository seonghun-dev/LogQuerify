use {
    crate::LocalFileStorage,
    async_trait::async_trait,
    gluesql::core::{
        data::{Key, Schema},
        error::Result,
        store::{DataRow, RowIter, Store},
    },
};

#[async_trait(?Send)]
impl Store for LocalFileStorage {
    async fn fetch_schema(&self, table_name: &str) -> Result<Option<Schema>> {
        todo!()
    }

    async fn fetch_all_schemas(&self) -> Result<Vec<Schema>> {
        todo!()
    }

    async fn fetch_data(&self, table_name: &str, key: &Key) -> Result<Option<DataRow>> {
        todo!()
    }

    async fn scan_data(&self, table_name: &str) -> Result<RowIter> {
        todo!()
    }
}
