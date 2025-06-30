use std::collections::HashMap;
use std::str::FromStr;
use std::rc::Rc;


use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_calendar::calendars::NoCalendar::NC;
use crate::terms::grp_calendar::calendars::MondayToFriday::MF;
use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util::Value::Value;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Calendar {
    NC(NC),
    MF(MF),
}

impl Calendar {
    /// Décrit l'état actuel de l'enum en appelant `presentation` si nécessaire
    pub fn description(&self) -> String {
        match self {
            Self::NC(NC) => NC.type_str(),
            Self::MF(MF) => MF.type_str()
        }
    }
    
    pub fn new(element: &str) -> Result<Self, ParseError> {
        Calendar::from_str(element)
    }
    
    pub fn provide_rc(string_map: &HashMap<String, Value>, key: &str) -> Option<Rc<Self>> {
        match string_map.get(key) {
            None => Some(Rc::new(Calendar::default())), // Clé absente : valeur par défaut dans un Some
            Some(s) => {
                match Self::from_str(s.as_string().unwrap().as_str()) {
                    Ok(calendar) => Some(Rc::new(calendar)), // Valeur valide
                    Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                }
            }
        }
    }

}

impl TraitBusinessDayCalendar for Calendar {
    fn is_business_day(&self, date: &IsoDatetime) -> bool {
        match self {
            Self::NC(NC) => NC.is_business_day(date),
            Self::MF(MF) => MF.is_business_day(date)
        }
    }
}

impl FromStr for Calendar {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "NC" => Ok(Self::NC(NC::new())),
            "MF" => Ok(Self::MF(MF::new())),
            _ => Err(ParseError {
                message: format!("Invalid Calendar cont_type: {}", s),
            }),
        }
    }
}

impl Default for Calendar {
    fn default() -> Self {
        Self::NC(NC::new())
    }
}

