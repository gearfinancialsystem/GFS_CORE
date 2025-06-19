

#[derive(Debug, Eq, PartialEq)]

pub struct MWH;

impl MWH {
    pub fn new() -> Self {
        return MWH;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

