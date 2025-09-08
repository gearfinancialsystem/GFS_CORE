#[derive(Debug, Eq, Clone, Copy, PartialEq)]

pub struct OUT;

impl OUT {
    pub fn new() -> Self {
        return OUT;
    }
    pub fn type_str(&self) -> String {
        return "OUT contract cont_type".to_string();
    }
}

