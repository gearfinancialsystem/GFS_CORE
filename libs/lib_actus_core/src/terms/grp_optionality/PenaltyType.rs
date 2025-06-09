use std::collections::HashMap;
use std::str::FromStr;
use crate::util::ParseError::ParseError;
use crate::terms::grp_optionality::penalty_type::N::N;
use crate::terms::grp_optionality::penalty_type::A::A;
use crate::terms::grp_optionality::penalty_type::R::R;
use crate::terms::grp_optionality::penalty_type::I::I;
use crate::traits::TraitTermDescription::TraitTermDescription;

#[derive(Debug, PartialEq, Eq)]
pub enum PenaltyType {
    N(N),
    A(A),
    R(R),
    I(I),
    None
}

impl PenaltyType {
    pub fn description(&self) -> String {
        match self {
            Self::N(N) => N.type_str(),
            Self::A(A) => A.type_str(),
            Self::R(R) => R.type_str(),
            Self::I(I) => I.type_str(),
            Self::None => "None".to_string(),
        }
    }
    pub fn new_N() -> Self {
        PenaltyType::N(N::new())
    }
    pub fn new_A() -> Self {
        PenaltyType::A(A::new())
    }
    pub fn new_R() -> Self {
        PenaltyType::R(R::new())
    }
    pub fn new_I() -> Self {
        PenaltyType::I(I::new())
    }
    pub fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Box<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                PenaltyType::from_str(s).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box
            .unwrap_or_default()
    }
}

impl FromStr for PenaltyType {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "N" => Ok(Self::new_N()),
            "A" => Ok(Self::new_A()),
            "R" => Ok(Self::new_R()),
            "I" => Ok(Self::new_I()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayConvention: {}", s)})
        }
    }
}

impl Default for PenaltyType {
    fn default() -> Self {
        Self::N(N)
    }
}

impl TraitTermDescription for PenaltyType {
    fn get_identifier(&self) -> &str {
        "penaltyType"
    }
    fn get_group(&self) -> &str {
        "Optionality"
    }
    fn get_name(&self) -> &str {
        "Penalty Type"
    }
    fn get_acronym(&self) -> &str {
        "PYTP"
    }
    fn get_type(&self) -> &str {
        "Enum"
    }
    fn get_allowed_values(&self) -> &str {
        "[{'option': '0', 'identifier': 'noPenalty', 'name': 'No Penalty', 'acronym': 'N', 'description': 'No penalty applies.\r'}, {'option': '1', 'identifier': 'fixedPenalty', 'name': 'Fixed Penalty', 'acronym': 'A', 'description': 'A fixed amount applies as penalty.\r'}, {'option': '2', 'identifier': 'relativePenalty', 'name': 'Relative Penalty', 'acronym': 'R', 'description': 'A penalty relative to the notional outstanding applies.\r'}, {'option': '3', 'identifier': 'interestRateDifferential', 'name': 'Interest Rate Differential', 'acronym': 'I', 'description': 'A penalty based on the current interest rate differential relative to the notional outstanding applies.'}]"
    }
    fn get_default_value(&self) -> &str {
        "O"
    }
    fn get_description(&self) -> &str {
        "Defines whether prepayment is linked to a penalty and of which kind."
    }
}  