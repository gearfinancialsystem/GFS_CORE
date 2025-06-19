

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct E;

impl E {
    pub fn new() -> Self {
        return E;
    }
    pub fn type_str(&self) -> String {
        return "E Scaling Effect".to_string();
    }
}

