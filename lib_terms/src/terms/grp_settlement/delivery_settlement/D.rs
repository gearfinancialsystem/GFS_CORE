use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct D;
impl D {
    pub fn new() -> Self {
        D
    }
}
impl fmt::Display for D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "DeliverySettlement: {}", D.to_string())

    }
}
