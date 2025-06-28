use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_optionality::option_exercise_type::A::A;
use crate::terms::grp_optionality::option_exercise_type::B::B;
use crate::terms::grp_optionality::option_exercise_type::E::E;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_fees::FeeBasis::FeeBasis;

#[derive(PartialEq, Eq, Debug)]
pub enum OptionExerciseType {
    E(E),
    B(B),
    A(A),
    None
}

impl OptionExerciseType {
    pub fn description(&self) -> String {
        match self {
            OptionExerciseType::E(E) => E.type_str(),
            OptionExerciseType::B(B) => B.type_str(),
            OptionExerciseType::A(A) => A.type_str(),
            OptionExerciseType::None => "".to_string(),
        }
    }

    pub fn new(element: Option<&str>) -> Result<Self, ParseError> {
        match element {
            Some(n) => OptionExerciseType::from_str(n),
            None => Ok(OptionExerciseType::None),
        }
    }
}

impl FromStr for OptionExerciseType {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "E" => Ok(Self::E(E::new())),
            "B" => Ok(Self::B(B::new())),
            "A" => Ok(Self::A(A::new())),
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s)})
        }
    }
}

impl Default for OptionExerciseType {
    fn default() -> Self {
        Self::None
    }
}


