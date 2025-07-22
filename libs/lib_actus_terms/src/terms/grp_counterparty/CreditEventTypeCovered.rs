use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use crate::terms::grp_counterparty::contract_performance::Df::DF;
use crate::terms::grp_counterparty::contract_performance::Dl::DL;
use crate::terms::grp_counterparty::contract_performance::Dq::DQ;
use crate::exceptions::ParseError::ParseError;
use crate::util::Value::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CreditEventTypeCoveredElement {
    DL(DL),
    DQ(DQ),
    DF(DF)
}
impl CreditEventTypeCoveredElement {
    pub fn new(value: &str) -> Result<Self, ParseError> {
        let a = CreditEventTypeCoveredElement::from_str(value);
        match a {
            Ok(a) => Ok(a),
            Err(e) => Err(e)
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            CreditEventTypeCoveredElement::DL(DL) => "DL".to_string(),
            CreditEventTypeCoveredElement::DQ(DQ) => "DQ".to_string(),
            CreditEventTypeCoveredElement::DF(DF) => "DF".to_string(),
            _ => "".to_string(),
        }
    }

}


impl FromStr for CreditEventTypeCoveredElement {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "DL" => Ok(CreditEventTypeCoveredElement::DL(DL::new())),
            "DQ" => Ok(CreditEventTypeCoveredElement::DQ(DQ::new())),
            "DF" => Ok(CreditEventTypeCoveredElement::DF(DF::new())),
            _ => Err(ParseError { message: format!("Invalid CreditEventTypeCoveredElement: {}", s) }),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreditEventTypeCovered(pub Vec<CreditEventTypeCoveredElement>);

impl CreditEventTypeCovered {

    pub fn new(value: &str) -> Result<Self, ParseError> {
        let a = CreditEventTypeCoveredElement::from_str(value);
        match a {
            Ok(a) => {
                let mut n = Vec::new();
                n.push(a);
                Ok(CreditEventTypeCovered(n))
            }
            Err(e) => return Err(e)
        }
    }
    pub fn values(&self) -> &Vec<CreditEventTypeCoveredElement> {
        &self.0
    }

    pub fn provide_from_input_dict(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        string_map.get(key).and_then(|s| {
            if let Some(values) = s.as_vec() {
                let parsed_v: Vec<CreditEventTypeCoveredElement> = values
                    .iter()
                    .filter_map(|v| v.as_string().and_then(|s| CreditEventTypeCoveredElement::from_str(&s).ok()))
                    .collect();

                if parsed_v.is_empty() {
                    None
                } else {
                    Some(CreditEventTypeCovered(parsed_v))
                }
            } else {
                None // Not a vector type
            }
        })
    }

    pub fn to_string_vec(&self) -> Vec<String> {
        self.0.iter().map(|elem| {
            match elem {
                CreditEventTypeCoveredElement::DL(_) => "DL".to_string(),
                CreditEventTypeCoveredElement::DQ(_) => "DQ".to_string(),
                CreditEventTypeCoveredElement::DF(_) => "DF".to_string(),
            }
        }).collect()
    }

    pub fn push(&mut self, element: CreditEventTypeCoveredElement) {
        self.0.push(element);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn contains(&self, element: &CreditEventTypeCoveredElement) -> bool {
        self.0.contains(element)
    }
}

impl Default for CreditEventTypeCovered {
    fn default() -> Self {
        CreditEventTypeCovered::new("DF").unwrap()
    }
}

impl fmt::Display for CreditEventTypeCovered {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for elem in &self.0 {
            match elem {
                CreditEventTypeCoveredElement::DL(dl) => writeln!(f, "CreditEventTypeCovered: {}", dl.to_string())?,
                CreditEventTypeCoveredElement::DQ(dq) => writeln!(f, "CreditEventTypeCovered: {}", dq.to_string())?,
                CreditEventTypeCoveredElement::DF(df) => writeln!(f, "CreditEventTypeCovered: {}", df.to_string())?,
            }
        }
        Ok(())
    }
}