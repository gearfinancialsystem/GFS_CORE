

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct DEC;

impl DEC {
    pub fn new() -> Self {
        DEC
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

