

#[derive(Debug, Eq, PartialEq)]
pub struct SWAPS;
impl SWAPS {
    pub fn new() -> Self {
        return SWAPS;
    }
    pub fn type_str(&self) -> String {
        return "SWAPS contract cont_type".to_string();
    }
}

