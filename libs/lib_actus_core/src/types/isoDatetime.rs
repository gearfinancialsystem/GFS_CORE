use std::collections::HashMap;
use std::ops::Add;
use std::ops::Sub;
use std::rc::Rc;
use chrono::{Days, Months, NaiveDateTime, NaiveDate, Datelike};
use crate::types::IsoPeriod::IsoPeriod;

pub type IsoDatetime = NaiveDateTime;

pub trait traitNaiveDateTimeExtension {
    fn double(&self) -> Self;
    fn is_last_day_of_month(&self) -> bool;
    fn provide_box(sm: &HashMap<String, String>, key: &str) -> Option<Box<NaiveDateTime>>;
    fn provide(string_map: &HashMap<String, String>, key: &str) -> Option<NaiveDateTime>;
    fn provide_rc(sm: &HashMap<String, String>, key: &str) -> Option<Rc<NaiveDateTime>>;
    fn provide_box_vec(sm: &HashMap<String, String>, key: &str) -> Option<Box<Vec<NaiveDateTime>>>;
    fn provide_vec(string_map: &HashMap<String, String>, key: &str) -> Option<Vec<NaiveDateTime>>;
}

impl traitNaiveDateTimeExtension for NaiveDateTime {
    fn double(&self) -> Self {
        *self
    }
    fn is_last_day_of_month(&self) -> bool {
        // Add one day to the given date
        let first_day_of_next_month = {
            if self.date().month() == 12 {
                NaiveDate::from_ymd_opt((self.date().year() + 1) as i32, 1, 1).unwrap()
            }
            else {
                NaiveDate::from_ymd_opt(self.date().year() as i32, self.date().month() + 1, 1).unwrap()
            }
        };
        let last_day_of_month = first_day_of_next_month.pred_opt().unwrap().day();
        if last_day_of_month == self.day() {
            true
        }
        else {
            false
        }

    }
    fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Option<Box<NaiveDateTime>> {
        string_map.get(key).and_then(|s| s.parse::<NaiveDateTime>().ok()).map(Box::new)
    }
    fn provide(string_map: &HashMap<String, String>, key: &str) -> Option<NaiveDateTime> {
        string_map.get(key).and_then(|s| s.parse::<NaiveDateTime>().ok())
    }
    fn provide_box_vec(string_map: &HashMap<String, String>, key: &str) -> Option<Box<Vec<NaiveDateTime>>> {
        string_map.get(key).and_then(|s| {
            let dates: Vec<NaiveDateTime> = s.split(',')
                .map(|date_str| date_str.trim())
                .filter_map(|date_str| date_str.parse::<NaiveDateTime>().ok())
                .collect();
            if dates.is_empty() {
                None
            } else {
                Some(Box::new(dates))
            }
        })
    }
    fn provide_vec(string_map: &HashMap<String, String>, key: &str) -> Option<Vec<NaiveDateTime>> {
        string_map.get(key).and_then(|s| {
            let dates: Vec<NaiveDateTime> = s.split(',')
                .map(|date_str| date_str.trim())
                .filter_map(|date_str| date_str.parse::<NaiveDateTime>().ok())
                .collect();
            if dates.is_empty() {
                None
            } else {
                Some(dates)
            }
        })
    }
    fn provide_rc(string_map: &HashMap<String, String>, key: &str) -> Option<Rc<Self>> {

        string_map.get(key).and_then(|s| s.parse::<NaiveDateTime>().ok()).map(|b| Rc::new(b))
    }

}

impl Add<IsoPeriod> for IsoDatetime {
    type Output = IsoDatetime;

    fn add(self, other: IsoPeriod) -> IsoDatetime {
        // Convert Yards to Meters and add to self
        self.checked_add_days(Days::new(other.days as u64)).unwrap()
            .checked_add_months(Months::new(other.months as u32)).unwrap()
            .checked_add_months(Months::new((other.years * 12) as u32)).unwrap()
    }
}

impl Sub<IsoPeriod> for IsoDatetime {
    type Output = IsoDatetime;

    fn sub(self, other: IsoPeriod) -> IsoDatetime {
        // Convert Yards to Meters and add to self
        self.checked_sub_days(Days::new(other.days as u64)).unwrap()
            .checked_sub_months(Months::new(other.months as u32)).unwrap()
            .checked_sub_months(Months::new((other.years * 12) as u32)).unwrap()
    }
}


#[cfg(test)]
mod tests_period_cycle_adjuster {
    use crate::time::adjusters::PeriodCycleAdjuster::PeriodCycleAdjuster;
    use crate::types::isoDatetime::IsoDatetime;
    use super::*;
}