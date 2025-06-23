use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_settlement::delivery_settlement::D::D;
use crate::terms::grp_settlement::delivery_settlement::S::S;
use crate::exceptions::ParseError::ParseError;
use crate::util::CommonUtils::Value;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum DeliverySettlement {
    S(S),
    D(D),
}

impl DeliverySettlement {
    /// Décrit l'état actuel de l'enum en appelant `presentation` si nécessaire
    pub fn description(&self) -> String {
        match self {
            DeliverySettlement::S(S) => S.type_str(),
            DeliverySettlement::D(D) => D.type_str(),
        }
    }

    pub fn new_S() -> Self {
        DeliverySettlement::S(S::new())
    }

    pub fn new_D() -> Self {
        DeliverySettlement::D(D::new())
    }

    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self>
    {
        match string_map.get(key) {
            None => Some(DeliverySettlement::default()), // Clé absente : valeur par défaut dans un Some
            Some(s) => {
                match DeliverySettlement::from_str(s.extract_string().unwrap().as_str()) {
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
            "" => Ok(DeliverySettlement::default()),
            "S" => Ok(DeliverySettlement::new_S()),
            "D" => Ok(DeliverySettlement::new_D()),
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
