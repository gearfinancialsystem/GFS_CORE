

#[derive(Debug, Eq, PartialEq)]

pub struct COL;

impl COL {
    pub fn new() -> Self {
        return COL;
    }
    pub fn type_str(&self) -> String {
        return "COL contract cont_type".to_string();
    }
    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

impl TraitEnumOptionDescription for COL {
    fn get_option_rank(&self) -> &str {
        "8"
    }
    fn get_identifier(&self) -> &str {
        "collateralPosition"
    }
    fn get_name(&self) -> &str {
        "Collateral Position"
    }
    fn get_acronym(&self) -> &str {
        "COL"
    }
    fn get_description(&self) -> &str {
        "Contract represents a collateral to an underlying instrument"
    }
}    