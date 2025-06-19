

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct UDL;

impl UDL {
    pub fn new() -> Self {
        return UDL;
    }
    pub fn type_str(&self) -> String {
        return "UDL contract cont_type".to_string();
    }
    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

