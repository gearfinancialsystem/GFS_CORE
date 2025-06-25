

#[derive(Clone, 
    Debug, Eq, PartialEq)]

pub struct NI;

impl NI {
    pub fn new() -> Self {
        return NI;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

