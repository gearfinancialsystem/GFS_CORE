use std::collections::HashMap;
use std::str::FromStr;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_interest::interestcalculationbase::Nt::NT;
use crate::terms::grp_interest::interestcalculationbase::Ntied::NTIED;
use crate::terms::grp_interest::interestcalculationbase::Ntl::NTL;
use crate::util::CommonUtils::Value;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum InterestCalculationBase {
    NT(NT),
    NTIED(NTIED),
    NTL(NTL)
}

impl InterestCalculationBase {
    pub fn new_NT() -> Self {
        InterestCalculationBase::NT(NT::new())
    }
    pub fn new_NTIED() -> Self {
        InterestCalculationBase::NTIED(NTIED::new())
    }
    pub fn new_NTL() -> Self {
        InterestCalculationBase::NTL(NTL::new())
    }
    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        crate::util::CommonUtils::CommonUtils::provide(string_map, key)
    }
}

impl Default for InterestCalculationBase {
    fn default() -> Self {
        InterestCalculationBase::new_NT()
    }
}

impl FromStr for InterestCalculationBase {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "NT" => Ok(InterestCalculationBase::new_NT()),
            "NTIED" => Ok(InterestCalculationBase::new_NTIED()),
            "NTL" => Ok(InterestCalculationBase::new_NTL()),
            "" => Ok(InterestCalculationBase::default()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s)})
        }
    }
}


