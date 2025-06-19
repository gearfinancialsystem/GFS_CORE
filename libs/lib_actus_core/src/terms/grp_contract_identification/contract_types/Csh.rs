

#[derive(Debug, Eq, PartialEq)]
pub struct CSH;

impl CSH {
    pub fn new() -> Self {
        return CSH;
    }
    pub fn type_str(&self) -> String {
        return "CSH contract cont_type".to_string();
    }
}

