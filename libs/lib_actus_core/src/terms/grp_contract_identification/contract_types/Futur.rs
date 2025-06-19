

#[derive(Debug, Eq, PartialEq)]
pub struct FUTUR;

impl FUTUR {
    pub fn new() -> Self {
        return FUTUR;
    }
    pub fn type_str(&self) -> String {
        return "FUTUR contract cont_type".to_string();
    }
}

