use crate::exceptions::ParseError::ParseError;
use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::process::Termination;
use std::str::FromStr;
use crate::terms::grp_calendar::eom_conventions::Eom::EOM;
use crate::terms::grp_calendar::eom_conventions::Sd::SD;
use crate::traits::TraitEndOfMonthConvention::TraitEndOfMonthConvention;
use crate::traits::TraitTermDescription::TraitTermDescription;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum EndOfMonthConvention {
    SD(SD),
    EOM(EOM)
}

impl EndOfMonthConvention {
    pub fn description(&self) -> String {
        match self {
            EndOfMonthConvention::SD(SD) => SD.type_str(),
            EndOfMonthConvention::EOM(EOM) => EOM.type_str()
        }
    }

    pub fn shift(&self, date: NaiveDateTime) -> NaiveDateTime {
        match self {
            EndOfMonthConvention::SD(SD) => SD.shift(&date),
            EndOfMonthConvention::EOM(EOM) => EOM.shift(&date)
        }
    }

    pub fn new_SD() -> Self {
        EndOfMonthConvention::SD(SD::new())
    }

    pub fn new_EOM() -> Self {
        EndOfMonthConvention::EOM(EOM::new())
    }

    pub fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Option<Box<Self>> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                Self::from_str(s).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box
            //.unwrap_or_default()
    }
    pub fn provide(string_map: &HashMap<String, String>, key: &str) -> Option<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                Self::from_str(s).ok()
            })
            .map(|b| b) // On stocke la convention dans une Box
        //.unwrap_or_default()
    }
}

impl FromStr for EndOfMonthConvention {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "SD" => Ok(Self::new_SD()),
            "EOM" => Ok(Self::new_EOM()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayConvention: {}", s)})
        }
    }
}

impl Default for EndOfMonthConvention {
    fn default() -> Self {
        Self::new_SD()
    }
}

