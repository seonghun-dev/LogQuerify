use {
    core::logger_type::LoggerType, gluesql::prelude::Glue,
    local_file_storage::local_file_storage::LocalFileStorage, std::path::Path,
};

// By specifying the log schema type, you determine which library's log format to parse.
// The logic for parsing the log schema varies depending on the log schema type.
//
// The parsed results of the logs are used to generate schemas.
// Provide the schema externally -> This includes LogBack configurations or file setup.
// Insert into a storage engine for use.
// Parse logs to generate schemas.

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    println!("current_dir: {:?}", current_dir);
    let storage = LocalFileStorage::new("localhost".to_string(), Path::new("./tmp").to_path_buf());

    let log_format = "[%d{yyyy-MM-dd HH:mm:ss}:%-3relative] %-5level ${PID:-} --- [%15.15thread] %-40.40logger{36} : %msg%n".to_string();
    let logger_type = LoggerType::LogBack;
    //let _schema = LogCustomSchema::new(logger_type, log_format).to_schema();

    let mut glue = Glue::new(storage);

    let payloads = glue.execute("SELECT * FROM test;").unwrap();
    println!("payloads: {:?}", payloads);
}
