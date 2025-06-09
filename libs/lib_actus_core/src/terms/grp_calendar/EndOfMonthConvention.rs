
use crate::util::ParseError::ParseError;
use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::process::Termination;
use std::str::FromStr;
use crate::terms::grp_calendar::eom_conventions::Eom::EOM;
use crate::terms::grp_calendar::eom_conventions::Sd::SD;
use crate::traits::TraitTermDescription::TraitTermDescription;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl TraitTermDescription for EndOfMonthConvention {
    fn get_identifier(&self) -> &str {
        "endOfMonthConvention"
    }
    fn get_group(&self) -> &str {
        "Calendar"
    }
    fn get_name(&self) -> &str {
        "End Of Month Convention"
    }
    fn get_acronym(&self) -> &str {
        "EOMC"
    }
    fn get_type(&self) -> &str {
        "Enum"
    }
    fn get_allowed_values(&self) -> &str {
        "[{'option': '0', 'identifier': 'sameDay', 'name': 'Same Day', 'acronym': 'SD', 'description': 'Schedule times always fall on the schedule anchor date day of the month.\r'}, {'option': '1', 'identifier': 'endOfMonth', 'name': 'End of Month', 'acronym': 'EOM', 'description': 'Schedule times fall on the end of every month if the anchor date represents the last day of the respective month.'}]"
    }
    fn get_default_value(&self) -> &str {
        "sd"
    }
    fn get_description(&self) -> &str {
        "When computing schedules a special problem arises if an anchor date is at the end of a month and a cycle of monthly or quarterly is applied (yearly in the case of leap years only). How do we have to interpret an anchor date April 30 plus 1M cycles? In case where EOM is selected, it will jump to the 31st of May, then June 30, July 31 and so on. If SM is selected, it will jump to the 30st always with of course an exception in February. This logic applies for all months having 30 or less days and an anchor date at the last day. Month with 31 days will at any rate jump to the last of the month if anchor date is on the last day."
    }
}    