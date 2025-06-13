use std::fmt::{Debug, Display};
use::std::error::Error;

#[derive(Debug)]
pub struct ContractTypeUnknownException;

impl Display for ContractTypeUnknownException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Contract cont_type unknown exception")
    }
}

impl Error for ContractTypeUnknownException {}
