

#[derive(Debug, Eq, PartialEq)]

pub struct M;

impl M {
    pub fn new() -> Self {
        return M;
    }
    pub fn type_str(&self) -> String {
        return "A Scaling Effect".to_string();
    }
}

