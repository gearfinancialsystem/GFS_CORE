use std::fmt;
use crate::terms::grp_settlement::delivery_settlement::D::D;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct S;

impl S {
    pub fn new() -> Self {
        S
    }

}

impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeliverySettlement: {}", S.to_string())
    }
}