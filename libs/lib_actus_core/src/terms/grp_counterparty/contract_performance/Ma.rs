use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct MA;

impl MA {
    pub fn new() -> Self {
        return MA;
    }

}

impl fmt::Display for MA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "MA")


    }
}