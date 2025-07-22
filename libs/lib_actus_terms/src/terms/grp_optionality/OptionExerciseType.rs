use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use crate::terms::grp_optionality::option_exercise_type::A::A;
use crate::terms::grp_optionality::option_exercise_type::B::B;
use crate::terms::grp_optionality::option_exercise_type::E::E;

use lib_actus_types::types::Value::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum OptionExerciseType {
    E(E),
    B(B),
    A(A),
    None
}

impl OptionExerciseType {


    pub fn new(element: Option<&str>) -> Result<Self, String> {
        match element {
            Some(n) => OptionExerciseType::from_str(n),
            None => Ok(OptionExerciseType::None),
        }
    }
    pub fn provide_from_input_dict(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        match string_map.get(key) {
            None => None,// A VERIFIER // Clé absente : valeur par défaut dans un Some
            Some(s) => {
                match Self::from_str(s.as_string().unwrap().as_str()) {
                    Ok(value) => Some(value), // Valeur valide
                    Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                }
            }
        }
    }
}

impl FromStr for OptionExerciseType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "E" => Ok(Self::E(E::new())),
            "B" => Ok(Self::B(B::new())),
            "A" => Ok(Self::A(A::new())),
            _ => Err(format!("Invalid BusinessDayAdjuster: {}", s))
        }
    }
}

impl Default for OptionExerciseType {
    fn default() -> Self {
        Self::None
    }
}

impl fmt::Display for OptionExerciseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::E(E) => write!(f, "OptionExerciseType: {}", E.to_string()),
            Self::B(B) => write!(f, "OptionExerciseType: {}", B.to_string()),
            Self::A(A) => write!(f, "OptionExerciseType: {}", A.to_string()),
            Self::None => write!(f, "OptionExerciseType: None"),
        }
    }
}
