

#[derive(Debug, Eq, PartialEq)]

pub struct MV;

impl MV {
    pub fn new() -> Self {
        return MV;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

