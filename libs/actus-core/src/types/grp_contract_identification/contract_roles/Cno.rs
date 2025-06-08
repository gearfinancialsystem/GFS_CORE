

#[derive(Debug, Eq, PartialEq)]

pub struct CNO;

impl CNO {
    pub fn new() -> Self {
        return CNO;
    }
    pub fn type_str(&self) -> String {
        return "CNO contract cont_type".to_string();
    }
    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

impl TraitEnumOptionDescription for CNO {
    fn get_option_rank(&self) -> &str {
        "9"
    }
    fn get_identifier(&self) -> &str {
        "closeOutNetting"
    }
    fn get_name(&self) -> &str {
        "Close out Netting"
    }
    fn get_acronym(&self) -> &str {
        "CNO"
    }
    fn get_description(&self) -> &str {
        "Contract creator and counterparty agree on netting payment obligations of underlying instruments in case of default."
    }
}