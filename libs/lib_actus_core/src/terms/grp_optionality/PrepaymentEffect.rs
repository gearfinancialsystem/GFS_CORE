use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_optionality::prepayment_effect::A::A;
use crate::terms::grp_optionality::prepayment_effect::M::M;
use crate::terms::grp_optionality::prepayment_effect::N::N;
use crate::terms::grp_reset_rate::ArrayFixedVariable::ArrayFixedVariable;
use crate::traits::TraitTermDescription::TraitTermDescription;
use crate::util::ParseError::ParseError;


#[derive(PartialEq, Eq)]
pub enum PrepaymentEffect {
    N(N),
    A(A),
    M(M)
}

impl PrepaymentEffect {
    pub fn description(&self) -> String {
        match self {
            PrepaymentEffect::N(F) => N.type_str(),
            PrepaymentEffect::A(A) => A.type_str(),
            PrepaymentEffect::M(M) => M.type_str(),
        }
    }
    pub fn new_N() -> Self {
        Self::N(N::new())
    }
    pub fn new_A() -> Self {
        Self::A(A::new())
    }
    pub fn new_M() -> Self {
        Self::M(M::new())
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

impl FromStr for PrepaymentEffect {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "N" => Ok(Self::new_N()),
            "A" => Ok(Self::new_A()),
            "M" => Ok(Self::new_M()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayConvention: {}", s)})
        }
    }
}

impl Default for PrepaymentEffect {
    fn default() -> Self {
        Self::new_N()
    }
}

impl TraitTermDescription for PrepaymentEffect {
    fn get_identifier(&self) -> &str {
        "prepaymentEffect"
    }
    fn get_group(&self) -> &str {
        "Optionality"
    }
    fn get_name(&self) -> &str {
        "Prepayment Effect"
    }
    fn get_acronym(&self) -> &str {
        "PPEF"
    }
    fn get_type(&self) -> &str {
        "Enum"
    }
    fn get_allowed_values(&self) -> &str {
        "[{'option': '0', 'identifier': 'noPrepayment', 'name': 'No Prepayment', 'acronym': 'N', 'description': 'Prepayment is not allowed under the agreement.\r'}, {'option': '1', 'identifier': 'prepaymentReducesRedemptionAmount', 'name': 'Prepayment Reduces Redemption Amount', 'acronym': 'A', 'description': 'Prepayment is allowed and reduces the redemption amount for the remaining period up to maturity.\r'}, {'option': '2', 'identifier': 'prepaymentReducesMaturity', 'name': 'Prepayment Reduces Maturity', 'acronym': 'M', 'description': 'Prepayment is allowed and reduces the maturity.'}]"
    }
    fn get_default_value(&self) -> &str {
        "N"
    }
    fn get_description(&self) -> &str {
        "This attribute defines whether or not the right of prepayment exists and if yes, how prepayment affects the remaining principal redemption schedule of the contract"
    }
}