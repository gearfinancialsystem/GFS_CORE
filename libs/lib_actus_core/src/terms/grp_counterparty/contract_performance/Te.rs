

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct TE;

impl TE {
    pub fn new() -> Self {
        return TE;
    }
    pub fn type_str(&self) -> String {
        return "TE contract cont_type".to_string();
    }
}


