#[derive(Clone, Debug, Eq, PartialEq)]

pub struct E;

impl E {
    pub fn new() -> Self { E }
    pub fn type_str(&self) -> String {
        "E Scaling Effect".to_string()
    }
}

