use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use crate::terms::grp_settlement::delivery_settlement::D::D;
use crate::terms::grp_settlement::delivery_settlement::S::S;
use crate::exceptions::ParseError::ParseError;
use crate::util::Value::Value;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum DeliverySettlement {
    S(S),
    D(D),
}

impl DeliverySettlement {

    
    pub fn new(element: &str) -> Result<Self, ParseError> {
        DeliverySettlement::from_str(element)
    }

    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self>
    {
        match string_map.get(key) {
            None => Some(DeliverySettlement::default()), // Clé absente : valeur par défaut dans un Some
            Some(s) => {
                match DeliverySettlement::from_str(s.as_string().unwrap().as_str()) {
                    Ok(value) => Some(value), // Valeur valide
                    Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                }
            }
        }
    }
    pub fn provide_from_input_dict(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        match string_map.get(key) {
            None => None,// A VERIFIER // Clé absente : valeur par défaut dans un Some
            Some(s) => {
                match Self::from_str(s.as_string().unwrap().as_str()) {
                    Ok(value) => Some(value), // Valeur valide
                    Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                }
            }
        }
    }
}

impl FromStr for DeliverySettlement {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "S" => Ok(DeliverySettlement::S(S::new())),
            "D" => Ok(DeliverySettlement::D(D::new())),
            _ => Err(ParseError {
                message: format!("Invalid Delivery Settlement: {}", s),
            }),
        }
    }
}

impl Default for DeliverySettlement {
    fn default() -> Self {
        DeliverySettlement::D(D)
    }
}

impl fmt::Display for DeliverySettlement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::S(S) => write!(f, "DeliverySettlement: {}", S.to_string()),
            Self::D(D) => write!(f, "DeliverySettlement: {}", D.to_string()),
        }
    }
}