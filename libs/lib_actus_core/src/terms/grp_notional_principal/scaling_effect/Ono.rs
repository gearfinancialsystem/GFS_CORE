

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct ONO;

impl ONO {
    pub fn new() -> Self {
        return ONO;
    }
    pub fn type_str(&self) -> String {
        return "ONO Scaling Effect".to_string();
    }
}

