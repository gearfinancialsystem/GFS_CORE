use std::collections::HashMap;
use std::str::FromStr;
use std::rc::Rc;
use std::fmt;


use crate::terms::grp_calendar::calendars::NoCalendar::NC;
use crate::terms::grp_calendar::calendars::MondayToFriday::MF;
// use crate::terms::grp_calendar::calendars::MondayToFridayWithHolidays::MFWH;

use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use lib_actus_types::types::Value::Value;
use crate::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Calendar {
    NC(NC),
    MF(MF),
    // MFWH(MFWH)
}

impl Calendar {
    
    pub fn new(element: &str) -> Result<Self, String> {
        Calendar::from_str(element)
    }
    
    pub fn provide_rc(string_map: &HashMap<String, Value>, key: &str) -> Rc<Self> {
        match string_map.get(key) {
            None => Rc::new(Calendar::default()), // Clé absente : valeur par défaut dans un Some
            Some(s) => {
                match Self::from_str(s.as_string().unwrap().as_str()) {
                    Ok(calendar) => Rc::new(calendar), // Valeur valide
                    Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                }
            }
        }
    }

}

impl TraitBusinessDayCalendar for Calendar {
    fn is_business_day(&self, date: &PhantomIsoDatetimeW) -> bool {
        match self {
            Self::NC(NC) => NC.is_business_day(date),
            Self::MF(MF) => MF.is_business_day(date),
            // Self::MFWH(MFWH) => MFWH.is_business_day(date)
        }
    }
}

impl FromStr for Calendar {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "NC" => Ok(Self::NC(NC::new())),
            "MF" => Ok(Self::MF(MF::new())),
            //"MFHW" => Ok(Self::MFWH(MFWH::new())),
            _ => Err(format!("Invalid Calendar cont_type: {}", s),
            ),
        }
    }
}

impl fmt::Display for Calendar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NC(nc) => write!(f, "{}", nc.to_string()),
            Self::MF(mf) => write!(f, "{}", mf.to_string()),
            //Self::MFWH(mfwh) => write!(f, "{}", mfwh.to_string()),
        }
    }
}

impl Default for Calendar {
    fn default() -> Self {
        Self::NC(NC::new())
    }
}

