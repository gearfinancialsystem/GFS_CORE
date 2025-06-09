use std::collections::HashMap;
use std::str::FromStr;
use std::rc::Rc;
use chrono::NaiveDateTime;

use crate::util::ParseError::ParseError;

use crate::terms::grp_calendar::calendars::NoCalendar::NC;
use crate::terms::grp_calendar::calendars::MondayToFriday::MF;
use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use crate::traits::TraitTermDescription::TraitTermDescription;

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
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                Self::from_str(s).ok()
            })
            .map(|b| Rc::new(b)) // On stocke la convention dans une Box
            // .unwrap_or_default()
    }
}

impl TraitBusinessDayCalendar for Calendar {

    fn is_business_day(&self, date: &NaiveDateTime) -> bool {
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
            "" => Ok(Calendar::default()),
            "NC" => Ok(Calendar::new_NC()),
            "MF" => Ok(Calendar::new_MF()),
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

impl TraitTermDescription for Calendar {
    fn get_identifier(&self) -> &str {
        "calendar"
    }
    fn get_group(&self) -> &str {
        "Calendar"
    }
    fn get_name(&self) -> &str {
        "Calendar"
    }
    fn get_acronym(&self) -> &str {
        "CLDR"
    }
    fn get_type(&self) -> &str {
        "Enum"
    }
    fn get_allowed_values(&self) -> &str {
        "[{'option': '0', 'identifier': 'noCalendar', 'name': 'No Calendar', 'acronym': 'NC', 'description': 'No holidays defined\r'}, {'option': '1', 'identifier': 'mondayToFriday', 'name': 'MondayToFriday', 'acronym': 'MF', 'description': 'Saturdays and Sundays are holidays'}]"
    }
    fn get_default_value(&self) -> &str {
        "noCalendar"
    }
    fn get_description(&self) -> &str {
        "Calendar defines the non-working days which affect the dates of contract events (CDE's) in combination with EOMC and BDC. Custom calendars can be added as additional enum options."
    }
}  