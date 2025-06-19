

#[derive(Debug, Eq, PartialEq)]
pub struct LAM;

impl LAM {
    pub fn new() -> Self {
        return LAM;
    }
    pub fn type_str(&self) -> String {
        return "LAM contract cont_type".to_string();
    }
}

