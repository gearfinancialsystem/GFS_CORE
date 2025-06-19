

#[derive(Debug, Eq, PartialEq)]

pub struct PND;

impl PND {
    pub fn new() -> Self {
        return PND;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}
