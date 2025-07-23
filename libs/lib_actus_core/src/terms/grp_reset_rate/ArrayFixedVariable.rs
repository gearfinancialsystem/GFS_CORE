use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_reset_rate::fixed_variable::F::F;
use crate::terms::grp_reset_rate::fixed_variable::V::V;

use crate::types::Value::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FixedVariableElement {
    F(F),
    V(V),
    None
}
impl FixedVariableElement {
    pub fn new(value: &str) -> Result<Self, String> {
        let a = FixedVariableElement::from_str(value);
        match a {
            Ok(a) => Ok(a),
            Err(e) => Err(e)
        }
    }

}

impl FromStr for FixedVariableElement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "F" => Ok(FixedVariableElement::F(F::new())),
            "V" => Ok(FixedVariableElement::V(V::new())),
            _ => Err(format!("Invalid CreditEventTypeCoveredElement: {}", s) ),
        }
    }
}
impl Default for FixedVariableElement {
    fn default() -> Self {
        Self::None
    }
}
#[derive(PartialEq, Debug, Clone)]
pub struct ArrayFixedVariable(Vec<FixedVariableElement>);


impl ArrayFixedVariable {
    pub fn values(&self) -> &Vec<FixedVariableElement> {
        &self.0
    }
    pub fn new(value: &str) -> Result<Self, String> {
        let a = FixedVariableElement::from_str(value);
        match a {
            Ok(a) => {
                let mut n = Vec::new();
                n.push(a);
                Ok(ArrayFixedVariable(n))
            }
            Err(e) => return Err(e)
        }
    }
    
    pub fn provide_from_input_dict(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        string_map.get(key).and_then(|s| {
            if let Some(values) = s.as_vec() {
                let parsed_v: Vec<FixedVariableElement> = values
                    .iter()
                    .filter_map(|v| v.as_string().and_then(|s| FixedVariableElement::from_str(&s).ok()))
                    .collect();

                if parsed_v.is_empty() {
                    None
                } else {
                    Some(ArrayFixedVariable(parsed_v))
                }
            } else {
                None // Not a vector type
            }
        })
    }
}





