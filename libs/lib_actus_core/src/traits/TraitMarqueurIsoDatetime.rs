
use crate::types::IsoDatetime::IsoDatetime;

pub trait TraitMarqueurIsoDatetime {

    fn value(&self) -> IsoDatetime;

    fn set_value(&mut self, value: &IsoDatetime);

    fn parse_from_string(s: &str, fmt: &str) -> Result<IsoDatetime, String>;
    
}