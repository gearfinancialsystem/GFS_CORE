use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

// Implement Display for ParseError
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.message)
    }
}

// Implement Error for ParseError
impl Error for ParseError {}

