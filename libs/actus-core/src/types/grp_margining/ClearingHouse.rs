use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_margining::clearing_house::N::N;
use crate::terms::grp_margining::clearing_house::Y::Y;
use crate::terms::grp_notional_principal::ArrayIncreaseDecrease::ArrayIncreaseDecrease;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;
use crate::util::ParseError::ParseError;


#[derive(PartialEq, Eq)]
pub enum ClearingHouse {
    Y(Y),
    N(N),
    None
}

impl ClearingHouse {
    pub fn description(&self) -> String {
        match self {
            Self::Y(Y) => Y.type_str(),
            Self::N(N) => N.type_str(),
            Self::None => "".to_string(),
        }
    }
    pub fn new_Y() -> Self {
        Self::Y(Y::new())
    }
    pub fn new_N() -> Self {
        Self::N(N::new())
    }
    pub fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Box<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                Self::from_str(s).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box
            .unwrap_or_default()
    }
}


impl Default for ClearingHouse {
    fn default() -> Self {
        Self::None
    }
}

impl FromStr for ClearingHouse {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "Y" => Ok(Self::new_Y()),
            "N" => Ok(Self::new_N()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayConvention: {}", s)})
        }
    }
}




impl TermDescriptionTrait for ClearingHouse {
    fn get_identifier(&self) -> &str {
        "clearingHouse"
    }
    fn get_group(&self) -> &str {
        "Margining"
    }
    fn get_name(&self) -> &str {
        "Clearing House"
    }
    fn get_acronym(&self) -> &str {
        "MRCLH"
    }
    fn get_type(&self) -> &str {
        "Enum"
    }
    fn get_allowed_values(&self) -> &str {
        "[{'option': '0', 'identifier': 'isClearingHouse', 'name': 'Is Clearing House', 'acronym': 'Y', 'description': 'Contract creator is the clearing house.\r'}, {'option': '1', 'identifier': 'isNotClearingHouse', 'name': 'Is Not Clearing House', 'acronym': 'N', 'description': 'Contract creator is not the clearing house.'}]"
    }
    fn get_default_value(&self) -> &str {
        ""
    }
    fn get_description(&self) -> &str {
        "Indicates wheter CRID takes a clearing house function or not. In other word, whether CRID receive margins (MRIM, MRVM)."
    }
}