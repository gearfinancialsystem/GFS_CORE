use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_notional_principal::increase_decrease::DEC::DEC;
use crate::terms::grp_notional_principal::increase_decrease::INC::INC;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_counterparty::contract_performance::Df::DF;
use crate::terms::grp_counterparty::contract_performance::Dl::DL;
use crate::terms::grp_counterparty::contract_performance::Dq::DQ;
use crate::terms::grp_counterparty::CreditEventTypeCovered::{CreditEventTypeCovered, CreditEventTypeCoveredElement};
use crate::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use crate::util::Value::Value;


#[derive(Clone, PartialEq, Eq, Debug)]
pub enum IncreaseDecreaseElement {
    INC(INC),
    DEC(DEC),
}


impl FromStr for IncreaseDecreaseElement {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "INC" => Ok(IncreaseDecreaseElement::INC(INC::new())),
            "DEC" => Ok(IncreaseDecreaseElement::DEC(DEC::new())),
            _ => Err(ParseError { message: format!("Invalid IncreaseDecreaseElement: {}", s) }),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ArrayIncreaseDecrease(Vec<IncreaseDecreaseElement>);

impl ArrayIncreaseDecrease {

    pub fn new(value: &str) -> Result<Self, ParseError> {
        let a = IncreaseDecreaseElement::from_str(value);
        match a {
            Ok(a) => {
                let mut n = Vec::new();
                n.push(a);
                Ok(ArrayIncreaseDecrease(n))
            }
            Err(e) => return Err(e)
        }
    }

    pub fn provide_from_input_dict(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        string_map.get(key).and_then(|s| {
            if let Some(values) = s.as_vec() {
                let parsed_inc_dec: Vec<IncreaseDecreaseElement> = values
                    .iter()
                    .filter_map(|v| v.as_string().and_then(|s| IncreaseDecreaseElement::from_str(&s).ok()))
                    .collect();

                if !parsed_inc_dec.is_empty() {
                    Some(ArrayIncreaseDecrease(parsed_inc_dec))
                } else {
                    None
                }
            } else {
                None // Not a vector type
            }
        })
    }

}
