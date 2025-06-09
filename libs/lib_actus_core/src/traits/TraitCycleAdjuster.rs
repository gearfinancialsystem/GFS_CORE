
use chrono::NaiveDateTime;

pub trait TraitCycleAdjuster {
    /// This adjuster adds a full cycle to a given time depending on the cycle definition.
    fn plus_cycle(&self, time: NaiveDateTime) -> NaiveDateTime;
    /// This adjuster deducts a full cycle to a given time depending on the cycle definition.
    fn minus_cycle(&self, time: NaiveDateTime) -> NaiveDateTime;
}
