pub mod error;
pub mod local_file_storage;
mod store;
mod store_mut;

use {
    gluesql::core::store::{
        AlterTable, CustomFunction, CustomFunctionMut, Index, IndexMut, Metadata, Transaction,
    },
    local_file_storage::LocalFileStorage,
};

impl CustomFunction for LocalFileStorage {}
impl CustomFunctionMut for LocalFileStorage {}
impl Index for LocalFileStorage {}
impl IndexMut for LocalFileStorage {}
impl Metadata for LocalFileStorage {}
impl AlterTable for LocalFileStorage {}
impl Transaction for LocalFileStorage {}
