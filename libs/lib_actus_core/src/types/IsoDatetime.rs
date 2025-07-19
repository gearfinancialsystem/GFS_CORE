use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, Deref};
use std::ops::Sub;
use std::rc::Rc;
use std::str::FromStr;
use chrono::{Days, Months, NaiveDateTime, NaiveDate, Datelike, Timelike, ParseResult, NaiveTime};
use chrono::format::Numeric::IsoYearDiv100;
use crate::types::IsoPeriod::IsoPeriod;
use crate::util::Value::Value;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;

// pub type IsoDatetime = NaiveDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IsoDatetime(pub NaiveDateTime);
pub trait TraitNaiveDateTimeExtension {
    fn is_last_day_of_month(&self) -> bool;
    fn last_date_of_month(&self) -> Self;
    fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<IsoDatetime>;
    fn provide_rc(sm: &HashMap<String, Value>, key: &str) -> Option<Rc<IsoDatetime>>;
    fn provide_vec(string_map: &HashMap<String, Value>, key: &str) -> Option<Vec<IsoDatetime>>;
}

impl IsoDatetime {

    pub fn new(dt_part: NaiveDate, tm_part: NaiveTime) -> Self {
        IsoDatetime(NaiveDateTime::new(
            dt_part,
            tm_part,
        ))
    }

    pub fn numdays_between_dates(&self, dt2: &IsoDatetime) -> f64 {
        (self.0 - dt2.0).num_days() as f64
    }
}

impl TraitNaiveDateTimeExtension for IsoDatetime {

    fn is_last_day_of_month(&self) -> bool {
        // Add one day to the given date
        let first_day_of_next_month = {
            if self.0.date().month() == 12 {
                NaiveDate::from_ymd_opt((self.0.date().year() + 1) as i32, 1, 1).unwrap()
            }
            else {
                NaiveDate::from_ymd_opt(self.0.date().year() as i32, self.0.date().month() + 1, 1).unwrap()
            }
        };
        let last_day_of_month = first_day_of_next_month.pred_opt().unwrap().day();
        if last_day_of_month == self.0.day() {
            true
        }
        else {
            false
        }

    }

    fn last_date_of_month(&self) -> IsoDatetime {
        let year = self.0.year();
        let month = self.0.month();
        let hour = self.0.hour();
        let minute = self.0.minute();
        let second = self.0.second();
        
        // Calculer le dernier jour du mois
        let next_month = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
        };

        let last_day_of_month = (next_month - NaiveDate::from_ymd_opt(year, month, 1).unwrap()).num_days() as u32;

        // Retourner la dernière date du mois avec l'heure à minuit
        IsoDatetime(NaiveDate::from_ymd_opt(year, month, last_day_of_month)
            .unwrap()
            .and_hms_opt(hour, minute, second)
            .unwrap())
    }

    fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<IsoDatetime> {
        string_map.get(key).and_then(|s| s.as_string().unwrap().parse::<NaiveDateTime>().ok().map(|dt| IsoDatetime(dt)))
    }

    fn provide_rc(string_map: &HashMap<String, Value>, key: &str) -> Option<Rc<IsoDatetime>> {

        string_map.get(key).and_then(|s| s.as_string().unwrap().parse::<NaiveDateTime>().ok()).map(|b| Rc::new(IsoDatetime(b)))
    }

    fn provide_vec(string_map: &HashMap<String, Value>, key: &str) -> Option<Vec<IsoDatetime>> {
        string_map.get(key).and_then(|s| {
            let dates: Vec<IsoDatetime> = s.as_string().unwrap().split(',')
                .map(|date_str| date_str.trim())
                .filter_map(|date_str| date_str.parse::<NaiveDateTime>().ok())
                .map(|dt| IsoDatetime(dt))
                .collect();
            if dates.is_empty() {
                None
            } else {
                Some(dates)
            }
        })
    }

}

impl TraitMarqueurIsoDatetime for IsoDatetime {
    fn value(&self) -> IsoDatetime {
        self.clone()
    }

    fn set_value(&mut self, valuex: &IsoDatetime) {
        *self = valuex.clone();
    }

    fn parse_from_string(s: &str, fmt: &str) -> Result<IsoDatetime, String> {
        match NaiveDateTime::parse_from_str(s, fmt) {
            Ok(dt) => Ok(IsoDatetime(dt)),
            Err(e) => Err(format!("{}", e)),
        }
    }
}

impl Add<IsoPeriod> for IsoDatetime {
    type Output = IsoDatetime;

    fn add(self, other: IsoPeriod) -> IsoDatetime {
        // Convert Yards to Meters and add to self
        IsoDatetime(self.0.checked_add_days(Days::new(other.days as u64)).unwrap()
            .checked_add_months(Months::new(other.months as u32)).unwrap()
            .checked_add_months(Months::new((other.years * 12) as u32)).unwrap())
    }
}

impl Sub<IsoPeriod> for IsoDatetime {
    type Output = IsoDatetime;

    fn sub(self, other: IsoPeriod) -> IsoDatetime {
        // Convert Yards to Meters and add to self
        IsoDatetime(self.0.checked_sub_days(Days::new(other.days as u64)).unwrap()
            .checked_sub_months(Months::new(other.months as u32)).unwrap()
            .checked_sub_months(Months::new((other.years * 12) as u32)).unwrap())
    }
}

impl fmt::Display for IsoDatetime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Formate la NaiveDateTime selon le format souhaité
        write!(f, "{}", self.0.format("%Y-%m-%dT%H:%M:%S"))
    }
}

impl Deref for IsoDatetime {
    type Target = NaiveDateTime;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl FromStr for IsoDatetime {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::parse_from_string(s, "%Y-%m-%dT%H:%M:%S")  {
            Ok(value) => Ok(value),
            Err(_) => Err(format!("Unable to parse {} as isodatetime", s)),
        }
    }
}