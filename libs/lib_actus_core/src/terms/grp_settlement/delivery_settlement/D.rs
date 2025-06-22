

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct D;
impl D {
    pub fn new() -> Self {
        D
    }
    pub fn type_str(&self) -> String {
        return "D".to_string();
    }
}

