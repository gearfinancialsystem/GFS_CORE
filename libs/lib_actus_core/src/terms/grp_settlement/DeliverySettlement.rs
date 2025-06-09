use std::{fmt, str::FromStr};
use crate::terms::grp_settlement::delivery_settlement::D::D;
use crate::terms::grp_settlement::delivery_settlement::S::S;
use crate::traits::TraitTermDescription::TraitTermDescription;
use crate::util::ParseError::ParseError;




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


impl TraitTermDescription for DeliverySettlement {
    fn get_identifier(&self) -> &str {
        "deliverySettlement"
    }
    fn get_group(&self) -> &str {
        "Settlement"
    }
    fn get_name(&self) -> &str {
        "Delivery Settlement"
    }
    fn get_acronym(&self) -> &str {
        "DS"
    }
    fn get_type(&self) -> &str {
        "Enum"
    }
    fn get_allowed_values(&self) -> &str {
        "[{'option': '0', 'identifier': 'cashSettlement', 'name': 'Cash Settlement', 'acronym': 'S', 'description': 'The market value of the underlying is settled.\r'}, {'option': '1', 'identifier': 'physicalSettlement', 'name': 'Physical Settlement', 'acronym': 'D', 'description': 'The underlying is delivered physically.\r'}]"
    }
    fn get_default_value(&self) -> &str {
        "D"
    }
    fn get_description(&self) -> &str {
        "Indicates whether the contract is settled in cash or physical delivery.
In case of physical delivery, the underlying contract and associated (future) cash flows are effectively exchanged. In case of cash settlement, the current market value of the underlying contract determines the cash flow exchanged."
    }
}