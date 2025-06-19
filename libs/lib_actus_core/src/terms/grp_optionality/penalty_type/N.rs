

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct N;

impl N {
    pub fn new() -> Self {
        return N;
    }
    pub fn type_str(&self) -> String {
        return "N Scaling Effect".to_string();
    }
}


