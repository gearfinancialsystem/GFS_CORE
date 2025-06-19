

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct DQ;

impl DQ {
    pub fn new() -> Self {
        return DQ;
    }
    pub fn type_str(&self) -> String {
        return "DQ contract cont_type".to_string();
    }
}

