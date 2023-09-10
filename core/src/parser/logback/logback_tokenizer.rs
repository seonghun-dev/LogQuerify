pub enum Token {
    Word(Word),
    PercentSign,
    LeftBrace,
    RightBrace,
    VerticalBar,
    Colon,
    Whitespace,
}

pub struct TokenWithPosition {
    token: Token,
    /// Location In Pattern, Means Line number starting from 1
    position: u64,
}

pub struct Word {
    pub value: String,
}

mod tests {

    #[test]
    fn test() {
        let actual = "[%d{yyyy-MM-dd HH:mm:ss}:%-3relative] %-5level ${PID:-} --- [%15.15thread] %-40.40logger{36} : %msg%n".to_owned();
    }
}
