

#[derive(Debug, Eq, PartialEq)]

pub struct RPA;

impl RPA {
    pub fn new() -> Self {
        return RPA;
    }
    pub fn type_str(&self) -> String {
        return "RPA contract cont_role".to_string();
    }
    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

impl TraitEnumOptionDescription for RPA {
    fn get_option_rank(&self) -> &str {
        "0"
    }
    fn get_identifier(&self) -> &str {
        "realPositionAsset"
    }
    fn get_name(&self) -> &str {
        "Real Position Asset"
    }
    fn get_acronym(&self) -> &str {
        "RPA"
    }
    fn get_description(&self) -> &str { 
        "Contract creator takes the asset or lender side."
    }
}   