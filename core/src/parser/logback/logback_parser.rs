enum LogBackKeyword {
    LoggerName,
    Date,
    Level,
    Thread,
    Relative,
    Line,
    Message,
    Method,
}

pub struct LogBackParser {
    tokens: Vec<LogBackKeyword>,
    format: String,
}
