

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct V;

impl V {
    pub fn new() -> Self {
        return V;
    }
    pub fn type_str(&self) -> String {
        return "Variable".to_string();
    }
}

