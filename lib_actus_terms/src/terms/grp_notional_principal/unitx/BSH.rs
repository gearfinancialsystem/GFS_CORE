use std::fmt;


#[derive(Debug, Eq, PartialEq)]

pub struct BSH;

impl BSH {
    pub fn new() -> Self {
        return BSH;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}
impl fmt::Display for BSH {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BSH")
    }
}