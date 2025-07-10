use std::fmt;


#[derive(Clone, Debug, Eq, PartialEq)]

pub struct UDL;

impl UDL {
    pub fn new() -> Self {
        return UDL;
    }

    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}
impl fmt::Display for UDL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UDL")
    }
}
