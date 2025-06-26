use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_reset_rate::fixed_variable::F::F;
use crate::terms::grp_reset_rate::fixed_variable::V::V;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_notional_principal::ArrayIncreaseDecrease::ArrayIncreaseDecrease;
use crate::util::CommonUtils::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArrayFixedVariable {
    F(F),
    V(V),
    None
}

impl ArrayFixedVariable {
    pub fn description(&self) -> String {
        match self {
            Self::F(F) => F.type_str(),
            Self::V(V) => V.type_str(),
            Self::None => "None".to_string(),
        }
    }
    pub fn new_F() -> Self {
        Self::F(F::new())
    }
    pub fn new_V() -> Self {
        Self::V(V::new())
    }
    pub fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Box<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                ArrayFixedVariable::from_str(s).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box
            .unwrap_or_default()
    }
    pub fn provide_vec(string_map: &HashMap<String, Value>, key: &str) -> Option<Vec<Self>> {
        match string_map.get(key) {
            None => None, // Clé absente : valeur par défaut dans un Some
            Some(s) => {

                let  a =  s.extract_vec_str().unwrap();
                let a2 = ArrayFixedVariable::from_str(a.get(0)?.as_str()).unwrap();

                let b0: Vec<ArrayFixedVariable> = a.iter().map(|s| {    ArrayFixedVariable::from_str(s.as_str()).unwrap()   }).collect();
                let b: Vec<Result<ArrayFixedVariable, ParseError>> = a.iter().map(|s| {    ArrayFixedVariable::from_str(s.as_str())   }).collect();
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
            "F" => Ok(Self::new_F()),
            "V" => Ok(Self::new_V()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s)})
        }
    }
}

impl Default for ArrayFixedVariable {
    fn default() -> Self {
        Self::None
    }
}


