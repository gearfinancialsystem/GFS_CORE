

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DF;


impl DF {
    pub fn new() -> Self {
        return DF;
    }
    pub fn type_str(&self) -> String {
        return "DF contract cont_type".to_string();
    }
}

