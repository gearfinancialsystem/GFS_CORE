use std::collections::HashMap;
use std::ops::Add;
use std::ops::Sub;
use std::rc::Rc;
use chrono::{Days, Months, NaiveDateTime, NaiveDate, Datelike, Timelike};
use crate::types::IsoPeriod::IsoPeriod;
use crate::util::Value::Value;

pub type IsoDatetime = NaiveDateTime;

pub trait TraitNaiveDateTimeExtension {
    fn double(&self) -> Self;
    fn is_last_day_of_month(&self) -> bool;
    fn last_date_of_month(&self) -> Self;
    fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<NaiveDateTime>;
    fn provide_rc(sm: &HashMap<String, Value>, key: &str) -> Option<Rc<NaiveDateTime>>;
    fn provide_vec(string_map: &HashMap<String, Value>, key: &str) -> Option<Vec<NaiveDateTime>>;
}

impl TraitNaiveDateTimeExtension for NaiveDateTime {
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
    fn last_date_of_month(&self) -> NaiveDateTime {
        let year = self.year();
        let month = self.month();
        let hour = self.hour();
        let minute = self.minute();
        let second = self.second();
        
        // Calculer le dernier jour du mois
        let next_month = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
        };

        let last_day_of_month = (next_month - NaiveDate::from_ymd_opt(year, month, 1).unwrap()).num_days() as u32;

        // Retourner la dernière date du mois avec l'heure à minuit
        NaiveDate::from_ymd_opt(year, month, last_day_of_month)
            .unwrap()
            .and_hms_opt(hour, minute, second)
            .unwrap()
    }

    fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<NaiveDateTime> {
        string_map.get(key).and_then(|s| s.as_string().unwrap().parse::<NaiveDateTime>().ok())
    }

    fn provide_vec(string_map: &HashMap<String, Value>, key: &str) -> Option<Vec<NaiveDateTime>> {
        string_map.get(key).and_then(|s| {
            let dates: Vec<NaiveDateTime> = s.as_string().unwrap().split(',')
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
    fn provide_rc(string_map: &HashMap<String, Value>, key: &str) -> Option<Rc<Self>> {

        string_map.get(key).and_then(|s| s.as_string().unwrap().parse::<NaiveDateTime>().ok()).map(|b| Rc::new(b))
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

