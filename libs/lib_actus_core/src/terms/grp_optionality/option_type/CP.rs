

#[derive(Debug, Eq, PartialEq)]

pub struct CP;

impl CP {
    pub fn new() -> Self {
        return CP;
    }
    pub fn type_str(&self) -> String {
        return "A Scaling Effect".to_string();
    }
}

