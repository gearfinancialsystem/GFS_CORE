// Define a trait for CycleAdjusterProvider similar to the Java interface

use crate::local_datetime::local_datetime::IsoDateTime;

trait TraitCycle {
    fn plus_cycle(&self, time: IsoDateTime) -> IsoDateTime;
    fn minus_cycle(&self, time: IsoDateTime) -> IsoDateTime;
}x