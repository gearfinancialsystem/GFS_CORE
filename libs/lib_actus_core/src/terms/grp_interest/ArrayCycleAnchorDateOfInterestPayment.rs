use crate::types::IsoDatetime::IsoDatetime;

#[derive(PartialEq, Debug, Clone)]
pub struct ArrayCycleAnchorDateOfInterestPayment(Vec<IsoDatetime>);