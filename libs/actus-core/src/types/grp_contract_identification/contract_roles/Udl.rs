

#[derive(Debug, Eq, PartialEq)]

pub struct UDL;

impl UDL {
    pub fn new() -> Self {
        return UDL;
    }
    pub fn type_str(&self) -> String {
        return "UDL contract cont_type".to_string();
    }
    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

impl TraitEnumOptionDescription for UDL {
    fn get_option_rank(&self) -> &str {
        "10"
    }
    fn get_identifier(&self) -> &str {
        "underlying"
    }
    fn get_name(&self) -> &str {
        "Underlying"
    }
    fn get_acronym(&self) -> &str {
        "UDL"
    }
    fn get_description(&self) -> &str {
        "Contract represents the underlying to a composed contract. Role of the underlying is derived from the parent."
    }
}    