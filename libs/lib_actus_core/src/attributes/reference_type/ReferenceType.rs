use std::str::FromStr;
use crate::attributes::reference_type::ReferenceType::ReferenceType::CNT;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_calendar::calendars::NoCalendar::NC;
use crate::terms::grp_contract_identification::ContractRole::ContractRole;

#[derive(Clone, Debug, PartialEq)]
pub enum ReferenceType {
    CNT,
    CID,
    MOC,
    EID,
    CST
}
impl ReferenceType {
    pub fn new_CNT() -> Self { Self::CNT }

    pub fn new_CID() -> Self { Self::CID }

    pub fn new_MOC() -> Self { Self::MOC }

    pub fn new_EID() -> Self { Self::EID }

    pub fn new_CST() -> Self { Self::CST }

}

impl FromStr for ReferenceType {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "CNT"  => Ok(Self::new_CNT()),
            "CID"  => Ok(Self::new_CID()),
            "MOC"  => Ok(Self::new_MOC()),
            "EID"  => Ok(Self::new_EID()),
            "CST"   => Ok(Self::new_CST()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s)})
        }
    }
}