use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::traits::DayCountConventionTrait::DayCountConventionTrait;
use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(PartialEq, Eq, Debug)]
pub struct A365;
impl A365 {
    pub fn new() -> Self {
        A365
    }
}

impl DayCountConventionTrait for A365 {
    /// Calculates the number of days between two dates
    fn day_count(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        (end_time - start_time).num_days() as f64
    }

    /// Calculates the day count fraction between two dates using the ISDA A/365-Fixed convention
    fn day_count_fraction(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        Self::day_count(&self, start_time, end_time) as f64 / 365.0
    }
}

impl TraitEnumOptionDescription for A365 {
    fn get_option_rank(&self) -> &str {
        "2"
    }
    fn get_identifier(&self) -> &str {
        "actualThreeSixtyFive"
    }
    fn get_name(&self) -> &str {
        "Actual Three Sixty Five"
    }
    fn get_acronym(&self) -> &str {
        "A365"
    }
    fn get_description(&self) -> &str {
        "Year fractions accrue on the basis of the actual number of days per month and 365 days per year in the respective period."
    }
}    