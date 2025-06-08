use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_boundary::boundary_direction::DECR::DECR;
use crate::terms::grp_boundary::boundary_direction::INCR::INCR;
use crate::terms::grp_boundary::boundary_leg_initially_active::FIL::FIL;
use crate::terms::grp_boundary::boundary_leg_initially_active::SEL::SEL;
use crate::terms::grp_boundary::BoundaryDirection::BoundaryDirection;

use crate::util::ParseError::ParseError;


#[derive(PartialEq, Eq, Debug)]
pub enum BoundaryLegInitiallyActive {
    FIL(FIL),
    SEL(SEL),
    None
}

impl BoundaryLegInitiallyActive {
    pub fn description(&self) -> String {
        match self {
            Self::FIL(FIL) => FIL.type_str(),
            Self::SEL(SEL) => SEL.type_str(),
            Self::None => "".to_string()
        }
    }
    pub fn new_FIL() -> Self {
        Self::FIL(FIL::new())
    }
    pub fn new_SEL() -> Self {
        Self::SEL(SEL::new())
    }

    pub fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Box<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                Self::from_str(s).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box
            .unwrap_or_default()
    }
}

impl FromStr for BoundaryLegInitiallyActive {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "FIL" => Ok(Self::new_FIL()),
            "SEL" => Ok(Self::new_SEL()),
            "NUll" => Ok(Self::None),
            _ => Err(ParseError { message: format!("Invalid BoundaryLegInitiallyActive: {}", s)})
        }
    }
}

impl Default for BoundaryLegInitiallyActive {
    fn default() -> Self {
        Self::None
    }
}

impl TermDescriptionTrait for BoundaryLegInitiallyActive {
    fn get_identifier(&self) -> &str {
        "boundaryLegInitiallyActive"
    }
    fn get_group(&self) -> &str {
        "Boundary"
    }
    fn get_name(&self) -> &str {
        "Boundary Leg Initially Active"
    }
    fn get_acronym(&self) -> &str {
        "BLIA"
    }
    fn get_type(&self) -> &str {
        "Enum"
    }
    fn get_allowed_values(&self) -> &str {
        "[{'option': '0', 'identifier': 'firstLeg', 'name': 'First Leg', 'acronym': 'FIL', 'description': 'the first leg is active when the boundary controlled switch contract is initialized.\r'}, {'option': '1', 'identifier': 'secondLeg', 'name': 'Second Leg', 'acronym': 'SEL', 'description': 'the second leg is active when the boundary controlled switch contract starts.\r'}, {'option': '2', 'identifier': 'Null', 'name': 'Null', 'acronym': 'Null', 'description': 'there is no active subcontract when the boundary controlled switch contract starts.'}]"
    }
    fn get_default_value(&self) -> &str {
        "there is no active subcontract"
    }
    fn get_description(&self) -> &str {
        "Specifies which leg - if any - is the active contract  in effect when the boundary controlled switch contract starts."
    }
}


