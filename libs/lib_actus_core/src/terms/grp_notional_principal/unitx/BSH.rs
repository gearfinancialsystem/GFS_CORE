

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
