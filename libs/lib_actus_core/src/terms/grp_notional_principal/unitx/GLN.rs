

#[derive(Debug, Eq, PartialEq)]

pub struct GLN;

impl GLN {
    pub fn new() -> Self {
        return GLN;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}
