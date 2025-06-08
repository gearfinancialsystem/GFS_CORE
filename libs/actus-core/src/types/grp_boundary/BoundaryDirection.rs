use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_boundary::boundary_direction::DECR::DECR;
use crate::terms::grp_boundary::boundary_direction::INCR::INCR;
use crate::terms::grp_counterparty::Seniority::Seniority;
use crate::traits::TraitTermDescription::TraitTermDescription;
use crate::util::ParseError::ParseError;



#[derive(PartialEq, Eq, Debug)]
pub enum BoundaryDirection {
    INCR(INCR),
    DECR(DECR),
    None
}

impl BoundaryDirection {
    pub fn description(&self) -> String {
        match self {
            Self::INCR(INCR) => DECR.type_str(),
            Self::DECR(DECR) => DECR.type_str(),
            Self::None => "".to_string()
        }
    }
    pub fn new_INCR() -> Self {
        Self::INCR(INCR::new())
    }
    pub fn new_DECR() -> Self {
        Self::DECR(DECR::new())
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

impl FromStr for BoundaryDirection {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "INCR" => Ok(Self::new_INCR()),
            "DECR" => Ok(Self::new_DECR()),
            _ => Err(ParseError { message: format!("Invalid BoundaryDirection: {}", s) })
        }
    }
}

impl Default for BoundaryDirection {
    fn default() -> Self {
        Self::None
    }
}

impl TermDescriptionTrait for BoundaryDirection {
    fn get_identifier(&self) -> &str {
        "boundaryDirection"
    }
    fn get_group(&self) -> &str {
        "Boundary"
    }
    fn get_name(&self) -> &str {
        "Boundary Direction"
    }
    fn get_acronym(&self) -> &str {
        "BDR"
    }
    fn get_type(&self) -> &str {
        "Enum"
    }
    fn get_allowed_values(&self) -> &str {
        "[{'option': '0', 'identifier': 'fromBelow', 'name': 'From Below', 'acronym': 'INCR', 'description': 'Boundary effect is trigerred if the observed underlying asset value is greater than or equal to the boundary value at a monitor time.\r'}, {'option': '1', 'identifier': 'fromAbove', 'name': 'From Above', 'acronym': 'DECR', 'description': 'Boundary action if observed market object value less than or equal to boundary value at a monitor time.'}]"
    }
    fn get_default_value(&self) -> &str {
        ""
    }
    fn get_description(&self) -> &str {
        "Boundary direction specifies the direction of motion in the underlying asset's price which will be considered a valid crossing of the boundary and trigger the boundary effect changing which, if any, of the boundary legs is  active."
    }
}