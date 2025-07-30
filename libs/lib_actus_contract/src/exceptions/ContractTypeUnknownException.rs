use std::fmt::{Debug, Display};
use std::error::Error;

// Votre énumération d'erreur définie
#[derive(Debug)]
pub enum ContractError {
    MissingTerms,
    UnknownContractType,
    Other(String),  // Ajouté pour gérer les messages d'erreur personnalisés
}

impl Display for ContractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContractError::MissingTerms => write!(f, "No contract terms provided"),
            ContractError::UnknownContractType => write!(f, "Contract cont_type unknown exception"),
            ContractError::Other(msg) => write!(f, "Contract error: {}", msg),
        }
    }
}

impl Error for ContractError {}

// Implémentation From pour convertir String en ContractError
impl From<String> for ContractError {
    fn from(s: String) -> Self {
        ContractError::Other(s)
    }
}

// Implémentation From pour convertir &str en ContractError
impl From<&str> for ContractError {
    fn from(s: &str) -> Self {
        ContractError::Other(s.to_string())
    }
}
