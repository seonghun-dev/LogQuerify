use {crate::logger_type::LoggerType, gluesql::core::ast::ColumnDef};

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
    pub fn to_schema(&self) -> Vec<ColumnDef> {
        todo!()
    }
}
