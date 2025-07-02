use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_reset_rate::fixed_variable::F::F;
use crate::terms::grp_reset_rate::fixed_variable::V::V;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_fees::FeeBasis::FeeBasis;
use crate::util::Value::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArrayFixedVariable {
    F(F),
    V(V),
    None
}

impl ArrayFixedVariable {

    
    pub fn new(element: Option<&str>) -> Result<Self, ParseError> {
        match element {
            Some(n) => ArrayFixedVariable::from_str(n),
            None => Ok(ArrayFixedVariable::None),
        }
    }

    pub fn provide_vec(string_map: &HashMap<String, Value>, key: &str) -> Option<Vec<Self>> {
        match string_map.get(key) {
            None => None, // Clé absente : valeur par défaut dans un Some
            Some(s) => {

                let  a =  s.as_vec().unwrap();
                //let a2 = ArrayFixedVariable::from_str(a.get(0)?.as_str()).unwrap();

                let b0: Vec<ArrayFixedVariable> = a.iter().map(|s| {    ArrayFixedVariable::from_str(s.to_string().as_str()).unwrap()   }).collect();
                let b: Vec<Result<ArrayFixedVariable, ParseError>> = a.iter().map(|s| {    ArrayFixedVariable::from_str(s.to_string().as_str())   }).collect();
                let c = b.iter().any(|r| r.is_err());

                if c == true {
                    panic!("Erreur de parsing pour la clé  avec la valeur ")
                } else {
                    Some(b0)
                }

            }
        }
    }
}

impl FromStr for ArrayFixedVariable {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "F" => Ok(Self::F(F::new())),
            "V" => Ok(Self::V(V::new())),
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s)})
        }
    }
}

impl Default for ArrayFixedVariable {
    fn default() -> Self {
        Self::None
    }
}


