use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::str::FromStr;
use gfs_lib_types::types::IsoDatetime::IsoDatetime;


pub trait TraitMarkerIsoDatetime
where
    Self: PartialEq + Clone + Copy + Hash + Debug + Display + FromStr + From<IsoDatetime>
{

    fn value(&self) -> IsoDatetime;

    fn set_value(&mut self, value: &IsoDatetime);

    fn parse_from_string(s: &str, fmt: &str) -> Result<IsoDatetime, String>;

    //fn to_phantom_type(&self) -> PhantomIsoDatetimeW;

    //fn to_schedule_time(&self) -> Option<ScheduleTime>;

    //fn to_start_time(&self) -> Option<StartTime>;

    //fn to_end_time(&self) -> Option<EndTime>;
}