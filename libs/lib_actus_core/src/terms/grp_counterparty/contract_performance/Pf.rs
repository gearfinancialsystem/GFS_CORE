

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct PF;

impl PF {
    pub fn new() -> Self {
        return PF;
    }
    pub fn type_str(&self) -> String {
        return "PF contract cont_type".to_string();
    }
}

