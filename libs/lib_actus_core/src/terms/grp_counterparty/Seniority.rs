use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_counterparty::guaranteed_exposure::MV::MV;
use crate::terms::grp_counterparty::guaranteed_exposure::NI::NI;
use crate::terms::grp_counterparty::guaranteed_exposure::NO::NO;
use crate::terms::grp_counterparty::GuaranteedExposure::GuaranteedExposure;
use crate::terms::grp_counterparty::seniority::J::J;
use crate::terms::grp_counterparty::seniority::S::S;
use crate::traits::TraitTermDescription::TraitTermDescription;
use crate::util::ParseError::ParseError;


#[derive(PartialEq, Eq, Debug)]
pub enum Seniority {
    S(S),
    J(J),
    None
}

impl Seniority {
    pub fn description(&self) -> String {
        match self {
            Self::S(S) => S.type_str(),
            Self::J(J) => J.type_str(),
            Self::None => "".to_string()
        }
    }
    pub fn new_S() -> Self {
        Self::S(S::new())
    }
    pub fn new_J() -> Self {
        Self::J(J::new())
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

impl FromStr for Seniority {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "S" => Ok(Self::new_S()),
            "J" => Ok(Self::new_J()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayConvention: {}", s)})
        }
    }
}

impl Default for Seniority {
    fn default() -> Self {
        Seniority::None
    }
}

impl TraitTermDescription for Seniority {
    fn get_identifier(&self) -> &str {
        "seniority"
    }
    fn get_group(&self) -> &str {
        "Counterparty"
    }
    fn get_name(&self) -> &str {
        "Seniority"
    }
    fn get_acronym(&self) -> &str {
        "SEN"
    }
    fn get_type(&self) -> &str {
        "Enum"
    }
    fn get_allowed_values(&self) -> &str {
        "[{'option': '0', 'identifier': 'senior', 'name': 'Senior', 'acronym': 'S', 'description': 'Contract represents senior debt.\r'}, {'option': '1', 'identifier': 'junior', 'name': 'Junior', 'acronym': 'J', 'description': 'Contract represents junior debt.'}]"
    }
    fn get_default_value(&self) -> &str {
        ""
    }
    fn get_description(&self) -> &str {
        "Refers to the order of repayment in the event of a sale or default of the issuer. "
    }
}