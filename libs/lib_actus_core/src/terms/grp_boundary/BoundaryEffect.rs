use std::collections::HashMap;
use std::str::FromStr;
use crate::util::ParseError::ParseError;

use crate::terms::grp_boundary::boundary_effect::Infil::INFIL;
use crate::terms::grp_boundary::boundary_effect::Insel::INSEL;
use crate::terms::grp_boundary::boundary_effect::Out::OUT;
use crate::traits::TraitTermDescription::TraitTermDescription;

#[derive(PartialEq, Eq, Debug)]
pub enum BoundaryEffect {
    INFIL(INFIL),
    INSEL(INSEL),
    OUT(OUT),
    None
}

impl BoundaryEffect {
    pub fn description(&self) -> String {
        match self {
            Self::INFIL(INFIL) => INFIL.type_str(),
            Self::INSEL(INSEL) => INSEL.type_str(),
            Self::OUT(OUT) => OUT.type_str(),
            Self::None => "None".to_string(),
        }
    }
    pub fn new_INFIL() -> Self {
        BoundaryEffect::INFIL(INFIL::new())
    }
    pub fn new_INSEL() -> Self {
        BoundaryEffect::INSEL(INSEL::new())
    }
    pub fn new_OUT() -> Self {
        BoundaryEffect::OUT(OUT::new())
    }

    pub fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Box<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                BoundaryEffect::from_str(s).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box
            .unwrap_or_default()
    }
}

impl FromStr for BoundaryEffect {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "INFIL" => Ok(BoundaryEffect::new_INFIL()),
            "INSEL" => Ok(BoundaryEffect::new_INSEL()),
            "OUT" => Ok(BoundaryEffect::new_OUT()),
            _ => Err(ParseError { message: format!("Invalid BoundaryEffect: {}", s)})
        }
    }
}

impl Default for BoundaryEffect {
    fn default() -> Self {
        Self::None
    }
}

impl TraitTermDescription for BoundaryEffect {
    fn get_identifier(&self) -> &str {
        "boundaryEffect"
    }
    fn get_group(&self) -> &str {
        "Boundary"
    }
    fn get_name(&self) -> &str {
        "Boundary Effect"
    }
    fn get_acronym(&self) -> &str {
        "BEF"
    }
    fn get_type(&self) -> &str {
        "Enum"
    }
    fn get_allowed_values(&self) -> &str {
        "[{'option': '0', 'identifier': 'knockINFirstLeg', 'name': 'KnockIN First Leg', 'acronym': 'INFIL', 'description': 'effect of boundary crossing is to knock IN the first leg making this the active contract; monitoring of the boundary stops.\r'}, {'option': '1', 'identifier': 'knockINSecondLeg', 'name': 'KnockIN Second Leg', 'acronym': 'INSEL', 'description': 'effect of boundary crossing is to knock IN the Second Leg making this the active contract; monitoring of the boundary stops.\r'}, {'option': '2', 'identifier': 'knockOUTCurrentLeg', 'name': 'KnockOUT Current Leg', 'acronym': 'OUT', 'description': 'effect of boundary crossing is to knockOUT any active contract so there is no active contract after the boundary crossing; monitoring of the boundary stops.'}]"
    }
    fn get_default_value(&self) -> &str {
        ""
    }
    fn get_description(&self) -> &str {
        "This term specifies which leg - if any-  becomes the active subcontract  when the underlying asset's price crosses the specified boundary value in the specified direction triggerring a boundary crossing event."
    }
}
