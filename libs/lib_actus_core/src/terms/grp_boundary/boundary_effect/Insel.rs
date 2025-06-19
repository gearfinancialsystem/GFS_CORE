#[derive(Debug, Eq, Clone, Copy, PartialEq)]

pub struct INSEL;

impl INSEL {
    pub fn new() -> Self {
        return INSEL;
    }
    pub fn type_str(&self) -> String {
        return "INSEL contract cont_type".to_string();
    }
}
