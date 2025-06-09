use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_optionality::option_exercise_type::A::A;
use crate::terms::grp_optionality::option_exercise_type::B::B;
use crate::terms::grp_optionality::option_exercise_type::E::E;
use crate::terms::grp_reset_rate::ArrayFixedVariable::ArrayFixedVariable;
use crate::traits::TraitTermDescription::TraitTermDescription;
use crate::util::ParseError::ParseError;


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
    pub fn new_E() -> Self {
        Self::E(E::new())
    }
    pub fn new_B() -> Self {
        Self::B(B::new())
    }
    pub fn new_A() -> Self {
        Self::A(A::new())
    }

    pub fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Box<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                OptionExerciseType::from_str(s).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box
            .unwrap_or_default()
    }
}

impl FromStr for OptionExerciseType {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "E" => Ok(Self::new_E()),
            "B" => Ok(Self::new_B()),
            "A" => Ok(Self::new_A()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayConvention: {}", s)})
        }
    }
}

impl Default for OptionExerciseType {
    fn default() -> Self {
        Self::None
    }
}

impl TraitTermDescription for OptionExerciseType {
    fn get_identifier(&self) -> &str {
        "optionExerciseType"
    }
    fn get_group(&self) -> &str {
        "Optionality"
    }
    fn get_name(&self) -> &str {
        "Option Exercise Type"
    }
    fn get_acronym(&self) -> &str {
        "OPXT"
    }
    fn get_type(&self) -> &str {
        "Enum"
    }
    fn get_allowed_values(&self) -> &str {
        "[{'option': '0', 'identifier': 'european', 'name': 'European', 'acronym': 'E', 'description': 'European-cont_type exercise.\r'}, {'option': '1', 'identifier': 'bermudan', 'name': 'Bermudan', 'acronym': 'B', 'description': 'Bermudan-cont_type exercise.\r'}, {'option': '2', 'identifier': 'american', 'name': 'American', 'acronym': 'A', 'description': 'American-cont_type exercise.'}]"
    }
    fn get_default_value(&self) -> &str {
        ""
    }
    fn get_description(&self) -> &str {
        "Defines whether the option is European (exercised at a specific date), American (exercised during a span of time) or Bermudan (exercised at certain points during a span of time)."
    }
}

