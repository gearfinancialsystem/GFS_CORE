

#[derive(Debug, Eq, PartialEq)]

pub struct INC;

impl INC {
    pub fn new() -> Self {
        INC
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

