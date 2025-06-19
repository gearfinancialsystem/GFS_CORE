use std::collections::HashMap;
use std::str::FromStr;
use std::rc::Rc;


use crate::exceptions::ParseError::ParseError;

use crate::terms::grp_calendar::calendars::NoCalendar::NC;
use crate::terms::grp_calendar::calendars::MondayToFriday::MF;
use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use crate::types::isoDatetime::IsoDatetime;

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

    pub fn new_NC() -> Self {
        Self::NC(NC::new())
    }

    pub fn new_MF() -> Self {
        Self::MF(MF::new())
    }

    pub fn provide_rc(string_map: &HashMap<String, String>, key: &str) -> Option<Rc<Self>> {
        match string_map.get(key) {
            None => Some(Rc::new(Calendar::default())), // Clé absente → valeur par défaut
            Some(s) => {
                match Self::from_str(s) {
                    Ok(calendar) => Some(Rc::new(calendar)),
                    Err(_) => None, // Valeur présente mais invalide
                }
            }
        }
    }
    pub fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Option<Box<Self>> {
        match string_map.get(key) {
            None => Some(Box::new(Calendar::default())), // Clé absente → valeur par défaut
            Some(s) => {
                match Self::from_str(s) {
                    Ok(calendar) => Some(Box::new(calendar)),
                    Err(_) => None, // Valeur présente mais invalide
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
            "" => Ok(Self::default()),
            "NC" => Ok(Self::new_NC()),
            "MF" => Ok(Self::new_MF()),
            _ => Err(ParseError {
                message: format!("Invalid Calendar cont_type: {}", s),
            }),
        }
    }
}

impl Default for Calendar {
    fn default() -> Self {
        Self::new_NC()
    }
}

