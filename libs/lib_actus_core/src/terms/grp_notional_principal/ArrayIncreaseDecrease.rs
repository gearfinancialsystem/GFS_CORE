use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_notional_principal::increase_decrease::DEC::DEC;
use crate::terms::grp_notional_principal::increase_decrease::INC::INC;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use crate::util::CommonUtils::Value;

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
    pub fn new_INC() -> Self {
        Self::INC(INC::new())
    }
    pub fn new_DEC() -> Self {
        Self::DEC(DEC::new())
    }
    pub fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Option<Box<Self>> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                Self::from_str(s).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box

    }
    // pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
    //     crate::util::CommonUtils::CommonUtils::provide(string_map, key)
    // }
    pub fn provide_vec(string_map: &HashMap<String, Value>, key: &str) -> Option<Vec<Self>> {
        match string_map.get(key) {
            None => None, // Clé absente : valeur par défaut dans un Some
            Some(s) => {

                let  a =  s.extract_vec_str().unwrap();
                let a2 = ArrayIncreaseDecrease::from_str(a.get(0)?.as_str()).unwrap();

                let b0: Vec<ArrayIncreaseDecrease> = a.iter().map(|s| {    ArrayIncreaseDecrease::from_str(s.as_str()).unwrap()   }).collect();
                let b: Vec<Result<ArrayIncreaseDecrease, ParseError>> = a.iter().map(|s| {    ArrayIncreaseDecrease::from_str(s.as_str())   }).collect();
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
            "F" => Ok(Self::new_INC()),
            "V" => Ok(Self::new_DEC()),
            _ => Err(ParseError { message: format!("Invalid ArrayIncreaseDecrease: {}", s)})
        }
    }
}

