
#[derive(Debug, Eq, PartialEq)]

pub struct DL;

impl DL {
    pub fn new() -> Self {
        return DL;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

