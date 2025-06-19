

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct MA;

impl MA {
    pub fn new() -> Self {
        return MA;
    }
    pub fn type_str(&self) -> String {
        return "MA contract cont_type".to_string();
    }
}

