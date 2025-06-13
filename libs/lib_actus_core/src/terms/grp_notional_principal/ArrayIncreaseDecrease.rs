use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_notional_principal::increase_decrease::DEC::DEC;
use crate::terms::grp_notional_principal::increase_decrease::INC::INC;
use crate::terms::grp_reset_rate::ArrayFixedVariable::ArrayFixedVariable;
use crate::traits::TraitTermDescription::TraitTermDescription;
use crate::exceptions::ParseError::ParseError;

#[derive(PartialEq, Eq, Debug)]
pub enum ArrayIncreaseDecrease {
    INC(INC),
    DEC(DEC),
    None
}

impl ArrayIncreaseDecrease {
    pub fn description(&self) -> String {
        match self {
            Self::INC(INC) => INC.type_str(),
            Self::DEC(DEC) => DEC.type_str(),
            Self::None => "".to_string(),
        }
    }
    pub fn new_INC() -> Self {
        Self::INC(INC::new())
    }
    pub fn new_DEC() -> Self {
        Self::DEC(DEC::new())
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

impl FromStr for ArrayIncreaseDecrease {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "F" => Ok(Self::new_INC()),
            "V" => Ok(Self::new_DEC()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayConvention: {}", s)})
        }
    }
}

impl Default for ArrayIncreaseDecrease {
    fn default() -> Self {
        Self::None
    }
}

