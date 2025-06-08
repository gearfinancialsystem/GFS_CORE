#[derive(Debug, Eq, PartialEq)]
pub struct BCS;

impl BCS {
    pub fn new() -> Self {
        return BCS;
    }
    pub fn type_str(&self) -> String {
        return "BCS contract cont_type".to_string();
    }
}
