use crate::types::isoDatetime::IsoDatetime;
use chrono::Datelike;
use crate::traits::TraitCountConvention::TraitDayCountConvention;
use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct E30360;
impl E30360 {
    pub fn new() -> Self {
        E30360
    }
}

impl TraitDayCountConvention for E30360 {
    /// Calculates the number of days between two dates using the ISDA 30E/360 convention
    fn day_count(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        let d1 = if start_time.day() == 31 { 30.0 } else { start_time.day() as f64 };
        let d2 = if end_time.day() == 31 { 30.0 } else { end_time.day() as f64 };

        let del_d = d2 - d1;
        let del_m = end_time.month() as i32 - start_time.month() as i32;
        let del_y = end_time.year() - start_time.year();

        (360.0 * del_y as f64) + (30.0 * del_m as f64) + del_d
    }

    /// Calculates the day count fraction based on the ISDA 30E/360 convention
    fn day_count_fraction(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        Self::day_count(&self, start_time, end_time) / 360.0
    }
}

