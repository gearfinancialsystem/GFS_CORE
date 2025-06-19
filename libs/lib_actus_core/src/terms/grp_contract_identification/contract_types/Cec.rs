

#[derive(Debug, Eq, PartialEq)]
pub struct CEC;

impl CEC {
    pub fn new() -> Self {
        return CEC;
    }
    pub fn type_str(&self) -> String {
        return "CEC contract cont_type".to_string();
    }
}

