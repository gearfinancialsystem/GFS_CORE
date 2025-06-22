

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct S;

impl S {
    pub fn new() -> Self {
        S
    }
    pub fn type_str(&self) -> String {
        return "S".to_string();
    }
}
