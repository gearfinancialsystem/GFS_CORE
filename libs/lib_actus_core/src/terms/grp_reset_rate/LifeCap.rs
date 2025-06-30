use std::ops::Deref;

#[derive(PartialEq, Debug, Clone)]
pub struct LifeCap(f64);


impl LifeCap {
    pub fn new(value: f64) -> Self {
        Self(value)
    }
}
// Implémentation de Deref pour LifeCap

impl Deref for LifeCap {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
