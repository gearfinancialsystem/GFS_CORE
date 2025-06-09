use std::collections::HashMap;
use std::str::FromStr;
use crate::util::ParseError::ParseError;

use crate::terms::grp_reset_rate::cycle_point_of_rate_reset::B::B;
use crate::terms::grp_reset_rate::cycle_point_of_rate_reset::E::E;
use crate::traits::TraitTermDescription::TraitTermDescription;

#[derive(Debug, Eq, PartialEq)]
pub enum CyclePointOfRateReset {
    B(B),
    E(E),
    None
}

impl CyclePointOfRateReset {
    pub fn description(&self) -> String {
        match self {
            CyclePointOfRateReset::B(B) => B.type_str(),
            CyclePointOfRateReset::E(E) => E.type_str(),
            CyclePointOfRateReset::None => "None".to_string(),
        }
    }
    pub fn new_B() -> Self {
        Self::B(B::new())
    }
    pub fn new_E() -> Self {
        Self::E(E::new())
    }
    pub fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Box<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                CyclePointOfRateReset::from_str(s).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box
            .unwrap_or_default()
    }
}

impl FromStr for CyclePointOfRateReset {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "B" => Ok(Self::new_B()),
            "E" => Ok(Self::new_E()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayConvention: {}", s)})
        }
    }
}

impl Default for CyclePointOfRateReset {
    fn default() -> Self {
        Self::B(B)
    }
}

impl TraitTermDescription for CyclePointOfRateReset {
    fn get_identifier(&self) -> &str {
        "cyclePointOfRateReset"
    }
    fn get_group(&self) -> &str {
        "Rate Reset"
    }
    fn get_name(&self) -> &str {
        "Cycle Point Of Rate Reset"
    }
    fn get_acronym(&self) -> &str {
        "RRPNT"
    }
    fn get_type(&self) -> &str {
        "Enum"
    }
    fn get_allowed_values(&self) -> &str {
        "[{'option': '0', 'identifier': 'beginning', 'name': 'Beginning', 'acronym': 'B', 'description': 'The new rate is applied at the beginning of the reset period.\r'}, {'option': '1', 'identifier': 'end', 'name': 'End', 'acronym': 'E', 'description': 'The new rate is applied at the end of the reset period.\r'}]"
    }
    fn get_default_value(&self) -> &str {
        "B"
    }
    fn get_description(&self) -> &str {
        "Normally rates get reset at the beginning of any resetting cycles. There are contracts where the rate is not set at the beginning but at the end of the cycle and then applied to the previous cycle (post-fixing); in other words the rate applies before it is fixed. Hence, the new rate is not known during the entire cycle where it applies. Therefore, the rate will be applied backwards at the end of the cycle. This happens through a correction of interest accrued."
    }
}   