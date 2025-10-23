use std::fmt;
use fmt::Display;
use fmt::Formatter;
use fmt::Result;
use std::io::Error as IoError;
use std::error::Error as StdError;
use crate::error::error_types::ErrorPayOffComputation::ErrorPayOffComputation;

#[derive(Debug)]

pub enum ErrorContractEnum {
    ErrorNetwork(String),
    ErrorFile(IoError),
    ErrorParsing,
    ErrorPayOffComputation(ErrorPayOffComputation),
}

impl Display for ErrorContractEnum {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ErrorContractEnum::ErrorNetwork(msg) => write!(f, "Erreur réseau : {}", msg),
            ErrorContractEnum::ErrorFile(e) => write!(f, "Erreur de fichier : {}", e),
            ErrorContractEnum::ErrorParsing => write!(f, "Erreur de parsing"),
            ErrorContractEnum::ErrorPayOffComputation(e) => write!(f, "Erreur de calcul de payoff : {}", e),
        }
    }
}

impl StdError for ErrorContractEnum {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            ErrorContractEnum::ErrorFile(e) => Some(e),
            _ => None,
        }
    }
}

// Implémenter From pour la conversion d'erreurs
impl From<IoError> for ErrorContractEnum {
    fn from(err: IoError) -> Self {
        ErrorContractEnum::ErrorFile(err)
    }
}