use std::fmt::{Display, Formatter, Result};
use std::error::Error;
use chrono::ParseError as ChronoParseError;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
}

impl ParseError {
    pub fn new(msg: String) -> Self {
        ParseError { message: msg }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Invalid Enum variant: {}", self.message)
    }
}

impl Error for ParseError {}

impl From<ChronoParseError> for ParseError {
    fn from(err: ChronoParseError) -> Self {
        // Convert the chrono::ParseError into your custom error cont_type
        ParseError { message: format!("Parse error: {}", err) }
    }
}
