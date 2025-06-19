

#[derive(Debug, Eq, PartialEq)]

pub struct A;

impl A {
    pub fn new() -> Self {
        return A;
    }
    pub fn type_str(&self) -> String {
        return "A Scaling Effect".to_string();
    }
}

