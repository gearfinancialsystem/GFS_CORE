#[derive(Debug, Eq, Clone, Copy, PartialEq)]

pub struct INFIL;

impl INFIL {
    pub fn new() -> Self {
        return INFIL;
    }
    pub fn type_str(&self) -> String {
        return "INFIL contract cont_type".to_string();
    }
}

