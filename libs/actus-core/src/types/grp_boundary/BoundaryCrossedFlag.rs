use crate::traits::TraitTermDescription::TraitTermDescription;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct BoundaryCrossedFlag(bool);

impl Default for BoundaryCrossedFlag {
    fn default() -> Self {
        BoundaryCrossedFlag(false)
    }
}

impl TraitTermDescription for BoundaryCrossedFlag {
    fn get_identifier(&self) -> &str {
        "boundaryCrossedFlag"
    }
    fn get_group(&self) -> &str {
        "Boundary"
    }
    fn get_name(&self) -> &str {
        "Boundary Crossed Flag"
    }
    fn get_acronym(&self) -> &str {
        "BCF"
    }
    fn get_type(&self) -> &str {
        "Boolean"
    }
    fn get_allowed_values(&self) -> &str {
        "[]"
    }
    fn get_default_value(&self) -> &str {
        "FALSE"
    }
    fn get_description(&self) -> &str {
        "Initializes the value of Boundary Crossed Flag state variable at statusDate"
    }
}    
