use crate::traits::TraitTermDescription::TraitTermDescription;

pub type BoundaryValue = f64;

impl TraitTermDescription for BoundaryValue {
    fn get_identifier(&self) -> &str {
        "boundaryValue"
    }
    fn get_group(&self) -> &str {
        "Boundary"
    }
    fn get_name(&self) -> &str {
        "Boundary Value"
    }
    fn get_acronym(&self) -> &str {
        "BV"
    }
    fn get_type(&self) -> &str {
        "Real"
    }
    fn get_allowed_values(&self) -> &str {
        "[]"
    }
    fn get_default_value(&self) -> &str {
        ""
    }
    fn get_description(&self) -> &str {
        "Boundary value in a barrier options contract, when reached, triggers the boundary effect specified e.g. Knock-In or Knock-out"
    }
}    
