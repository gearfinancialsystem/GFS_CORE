

#[derive(Debug, Eq, PartialEq)]

pub struct STN;

impl STN {
    pub fn new() -> Self {
        return STN;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

