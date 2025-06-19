#[derive(Debug, Eq, PartialEq)]

pub struct FIL;

impl FIL {
    pub fn new() -> Self {
        return FIL;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

