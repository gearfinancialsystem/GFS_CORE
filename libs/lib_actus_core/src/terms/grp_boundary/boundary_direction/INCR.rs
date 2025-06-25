#[derive(Clone, Debug, Eq, PartialEq)]

pub struct INCR;

impl INCR {
    pub fn new() -> Self {
        return INCR;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}
