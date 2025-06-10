// Define a trait for CycleAdjusterProvider similar to the Java interface

use crate::local_datetime::local_datetime::IsoDateTime;

pub trait TraitCycleAjuster {
    fn plus_cycle(&self, time: IsoDateTime) -> IsoDateTime;
    fn minus_cycle(&self, time: IsoDateTime) -> IsoDateTime;
}