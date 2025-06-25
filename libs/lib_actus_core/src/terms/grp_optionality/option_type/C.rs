

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct C;

impl C {
    pub fn new() -> Self {
        return C;
    }
    pub fn type_str(&self) -> String {
        return "A Scaling Effect".to_string();
    }
}
