
use crate::types::IsoCycle::IsoCycle;

pub trait TraitMarqueurIsoCycle {

    fn value(&self) -> &IsoCycle;

    fn set_value(&mut self, value: &IsoCycle);

}