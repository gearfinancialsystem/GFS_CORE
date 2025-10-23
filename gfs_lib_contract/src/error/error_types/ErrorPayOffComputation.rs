
use gfs_lib_terms::error::ErrorTerms::ErrorTermsEnum;
use crate::error::ErrorContract::ErrorContractEnum;
use std::fmt;
use fmt::Display;
use fmt::Formatter;
use fmt::Result;
use std::io::Error as IoError;
use std::error::Error as StdError;

// liste les raisons pour lesquelles le calcul du payoff peuvent foirer
#[derive(Debug)]
pub enum ErrorPayOffComputation {
    ErrorTerms(ErrorTermsEnum)
}

impl Display for ErrorPayOffComputation {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ErrorPayOffComputation::ErrorTerms(e) => write!(f, "Erreur de calcul de payoff : {}", e),
        }
    }
}

impl StdError for ErrorPayOffComputation {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            _ => None,
        }
    }
}