
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DECR;

impl DECR {
    pub fn new() -> Self {
        return DECR;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

