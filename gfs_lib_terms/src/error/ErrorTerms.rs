use std::fmt;
use fmt::Display;
use fmt::Formatter;
use fmt::Result;
use std::error::Error as StdError;

#[derive(Debug)]
pub enum ErrorTermsEnum {
    ErrorTermCreation(String),
    ErrorParsing,
}

impl Display for ErrorTermsEnum {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ErrorTermsEnum::ErrorTermCreation(msg) => write!(f, "Term creation error : {}", msg),
            ErrorTermsEnum::ErrorParsing => write!(f, "Erreur de parsing"),
        }
    }
}

impl StdError for ErrorTermsEnum {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            _ => None,
        }
    }
}




