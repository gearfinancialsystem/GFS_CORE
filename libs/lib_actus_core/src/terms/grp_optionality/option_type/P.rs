

#[derive(Debug, Eq, PartialEq)]

pub struct P;

impl P {
    pub fn new() -> Self {
        return P;
    }
    pub fn type_str(&self) -> String {
        return "A Scaling Effect".to_string();
    }
}

