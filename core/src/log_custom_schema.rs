use gluesql::core::data::Schema;

use crate::logger_type::LoggerType;

pub struct LogCustomSchema {
    pub schema_type: LoggerType,
    pub format: String,
}

impl LogCustomSchema {
    pub fn new(schema_type: LoggerType, format: String) -> Self {
        Self {
            schema_type,
            format,
        }
    }
    pub fn to_schema(&self) -> Schema {
        todo!()
    }
}
