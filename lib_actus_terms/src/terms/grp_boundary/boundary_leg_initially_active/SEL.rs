#[derive(Clone, Debug, Eq, PartialEq)]

pub struct SEL;

impl SEL {
    pub fn new() -> Self {
        return SEL;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

