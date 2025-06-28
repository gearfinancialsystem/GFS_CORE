use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_notional_principal::increase_decrease::DEC::DEC;
use crate::terms::grp_notional_principal::increase_decrease::INC::INC;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use crate::util::Value::Value;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ArrayIncreaseDecrease {
    INC(INC),
    DEC(DEC),
}

impl ArrayIncreaseDecrease {
    pub fn description(&self) -> String {
        match self {
            Self::INC(INC) => INC.type_str(),
            Self::DEC(DEC) => DEC.type_str(),
        }
    }

    pub fn new(element: &str) -> Result<Self, ParseError> {
        ArrayIncreaseDecrease::from_str(element)
    }
    
    pub fn provide_vec(string_map: &HashMap<String, Value>, key: &str) -> Option<Vec<Self>> {
        match string_map.get(key) {
            None => None, // Clé absente : valeur par défaut dans un Some
            Some(s) => {

                let  a =  s.as_vec().unwrap();
                //let a2 = ArrayIncreaseDecrease::from_str(a.get(0)?.as_str()).unwrap();

                let b0: Vec<ArrayIncreaseDecrease> = a.iter().map(|s| {    ArrayIncreaseDecrease::from_str(s.to_string().as_str()).unwrap()   }).collect();
                let b: Vec<Result<ArrayIncreaseDecrease, ParseError>> = a.iter().map(|s| {    ArrayIncreaseDecrease::from_str(s.to_string().as_str())   }).collect();
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

impl FromStr for ArrayIncreaseDecrease {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "INC" => Ok(Self::INC(INC::new())),
            "DEC" => Ok(Self::DEC(DEC::new())),
            _ => Err(ParseError { message: format!("Invalid ArrayIncreaseDecrease: {}", s)})
        }
    }
}

