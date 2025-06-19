

#[derive(Debug, Eq, PartialEq)]

pub struct NO;

impl NO {
    pub fn new() -> Self {
        return NO;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

