use std::{collections::HashMap, str::FromStr};
use crate::terms::grp_fees::fee_basis::A::A;
use crate::terms::grp_fees::fee_basis::N::N;
use crate::terms::grp_margining::ClearingHouse::ClearingHouse;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;
use crate::util::ParseError::ParseError;

#[derive(PartialEq, Eq, Debug)]
pub enum FeeBasis {
    A(A),
    N(N),
    None
}

impl FeeBasis {
    pub fn description(&self) -> String {
        match self {
            Self::A(A) => A.type_str(),
            Self::N(N) => N.type_str(),
            Self::None => "".to_string()
        }
    }
    pub fn new_A() -> Self {
        Self::A(A::new())
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


impl FromStr for FeeBasis {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" => Ok(Self::new_A()),
            "N" => Ok(Self::new_N()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayConvention: {}", s)})
        }
    }
}

impl Default for FeeBasis {
    fn default() -> Self {
        FeeBasis::None
    }
}

impl TermDescriptionTrait for FeeBasis {
    fn get_identifier(&self) -> &str {
        "feeBasis"
    }
    fn get_group(&self) -> &str {
        "Fees"
    }
    fn get_name(&self) -> &str {
        "Fee Basis"
    }
    fn get_acronym(&self) -> &str {
        "FEB"
    }
    fn get_type(&self) -> &str {
        "Enum"
    }
    fn get_allowed_values(&self) -> &str {
        "[{'option': '0', 'identifier': 'absoluteValue', 'name': 'Absolute Value', 'acronym': 'A', 'description': 'The fee rate represents an absolute value.\r'}, {'option': '1', 'identifier': 'nonimalValueOfTheUnderlying', 'name': 'Nominal Value of the Underlying', 'acronym': 'N', 'description': 'The fee rate represents a rate that accrues fees on the basis of the nominal value of the underlying.'}]"
    }
    fn get_default_value(&self) -> &str {
        ""
    }
    fn get_description(&self) -> &str {
        "Basis, on which Fee is calculated. For FEB=’A’, FER is interpreted as an absolute amount to be paid at every FP event and for FEB=’N’, FER represents a rate at which FP amounts accrue on the basis of the contract’s NT."
    }
}   