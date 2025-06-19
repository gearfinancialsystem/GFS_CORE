

#[derive(Debug, Eq, PartialEq, Copy, Clone)]

pub struct S;

impl S {
    pub fn new() -> Self {
        return S;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

