use std::{fmt, str::FromStr};
use crate::terms::grp_settlement::delivery_settlement::D::D;
use crate::terms::grp_settlement::delivery_settlement::S::S;
use crate::traits::TraitTermDescription::TraitTermDescription;
use crate::exceptions::ParseError::ParseError;


#[derive(Debug, Eq, PartialEq)]
pub enum DeliverySettlement {
    S(S),
    D(D),
    None
}

impl DeliverySettlement {
    /// Décrit l'état actuel de l'enum en appelant `presentation` si nécessaire
    pub fn description(&self) -> String {
        match self {
            DeliverySettlement::S(S) => S.type_str(),
            DeliverySettlement::D(D) => D.type_str(),
            DeliverySettlement::None => "".to_string()
        }
    }

    pub fn new_S() -> Self {
        DeliverySettlement::S(S::new())
    }

    pub fn new_D() -> Self {
        DeliverySettlement::D(D::new())
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
                message: format!("Invalid Calendar cont_type: {}", s),
            }),
        }
    }
}

impl Default for DeliverySettlement {
    fn default() -> Self {
        DeliverySettlement::None
    }
}
