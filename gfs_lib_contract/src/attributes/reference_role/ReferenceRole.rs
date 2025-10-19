use std::str::FromStr;
use crate::exceptions::ParseError::ParseError;


#[derive(Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ReferenceRole {
    UDL,
    FIL,
    SEL,
    COVE,
    COVI,
    externalReferenceIndex
}

impl ReferenceRole {
    pub fn new_UDL() -> Self { Self::UDL }

    pub fn new_FIL() -> Self { Self::FIL }

    pub fn new_SEL() -> Self { Self::SEL }

    pub fn new_COVE() -> Self { Self::COVE }

    pub fn new_COVI() -> Self { Self::COVI }

    pub fn to_stringx(&self) -> Result<String, ParseError> {
        match self {
            Self::UDL => Ok("UDL".to_string()),
            Self::FIL => Ok("FIL".to_string()),
            Self::SEL => Ok("SEL".to_string()),
            Self::COVE => Ok("COVE".to_string()),
            Self::COVI => Ok("COVI".to_string()),
            _ => Err(ParseError { message: format!("Invalid TOSTRING ContractPerformance ")})
        }

    }
}

impl FromStr for ReferenceRole {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "UDL"  => Ok(Self::new_UDL()),
            "FIL"  => Ok(Self::new_FIL()),
            "SEL"  => Ok(Self::new_SEL()),
            "COVE"  => Ok(Self::new_COVE()),
            "COVI"   => Ok(Self::new_COVI()),
            _ => Err(ParseError { message: format!("Invalid Reference Role: {}", s)})
        }
    }
}