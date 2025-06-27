use std::collections::HashMap;
use chrono::{Months, NaiveDateTime};
use std::hash::{Hash, Hasher};
use chrono::{Datelike, NaiveDate, Duration};
use regex::Regex;
use std::fmt;
use std::ops::Add;
use std::ops::Sub;
use crate::types::isoDatetime::IsoDatetime;
use chrono::Days;
use crate::util::CommonUtils::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsoPeriod {
    pub years: i32,
    pub months: i32,
    pub days: i32,
}


impl IsoPeriod {
    // Constructor for creating a new IsoPeriod
    pub fn new(years: i32, months: i32, days: i32) -> Self {
        IsoPeriod { years, months, days }
    }

    // Convert a IsoPeriod to an IsoDateTime by adding it to a reference NaiveDateTime
    pub fn to_iso_datetime(&self, reference: IsoDatetime) -> Option<IsoDatetime> {
        // Add years and months to the date part
        let new_date = reference.date()
            .with_year(reference.year() + self.years)?
            .with_month(reference.month() + self.months as u32)?;

        // Add days to the date
        let date_with_days = new_date.checked_add_days(chrono::Days::new(self.days as u64))?;

        // Combine the new date with the original time
        let new_datetime = NaiveDateTime::new(
            date_with_days,
            reference.time(),
        );

        Some(new_datetime)
    }

    // Adds this IsoPeriod to the specified temporal object (NaiveDate in this case)
    pub fn add_to(&self, date: NaiveDate) -> NaiveDate {
        date.checked_add_signed(Duration::days(self.days as i64)).unwrap()
            .checked_add_months(chrono::Months::new(self.months as u32)).unwrap()
            .with_year(date.year() + self.years).unwrap()
    }

    // Obtains a IsoPeriod consisting of the number of years, months, and days between two dates
    pub fn between(start_date: NaiveDate, end_date: NaiveDate) -> Self {
        let years = end_date.year() - start_date.year();
        let months = end_date.month() as i32 - start_date.month() as i32;
        let days = end_date.day() as i32 - start_date.day() as i32;

        // Adjust for negative months or days
        let (years, months, days) = if days < 0 {
            let last_day_of_previous_month = start_date.with_day(1).unwrap().pred_opt().unwrap().day();
            (years - (months < 0) as i32, months - 1 + 12 * (months < 0) as i32, days + last_day_of_previous_month as i32)
        } else {
            (years - (months < 0) as i32, months + 12 * (months < 0) as i32, days)
        };

        IsoPeriod { years, months, days }
    }

    // Checks if this IsoPeriod is equal to another IsoPeriod
    pub fn equals(&self, other: &IsoPeriod) -> bool {
        self.years == other.years && self.months == other.months && self.days == other.days
    }

    // Obtains an instance of IsoPeriod from a temporal amount (simplified)
    pub fn from(amount: &IsoPeriod) -> Self {
        IsoPeriod::new(amount.years, amount.months, amount.days)
    }

    // Gets the value of the requested unitx
    pub fn get(&self, unit: &str) -> Option<i32> {
        match unit {
            "years" => Some(self.years),
            "months" => Some(self.months),
            "days" => Some(self.days),
            _ => None,
        }
    }

    // Gets the chronology of this IsoPeriod, which is the ISO calendar system
    pub fn get_chronology(&self) -> String {
        "ISO".to_string()
    }
    // Gets the amount of days of this IsoPeriod
    pub fn get_days(&self) -> i32 {
        self.days
    }

    // Gets the amount of months of this IsoPeriod
    pub fn get_months(&self) -> i32 {
        self.months
    }

    // Gets the set of units supported by this IsoPeriod
    pub fn get_units(&self) -> Vec<String> {
        vec!["Years".to_string(), "Months".to_string(), "Days".to_string()]
    }

    // Gets the amount of years of this IsoPeriod
    pub fn get_years(&self) -> i32 {
        self.years
    }

    // Computes a hash code for this IsoPeriod
    pub fn hash_code(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    // Checks if any of the three units of this IsoPeriod are negative
    pub fn is_negative(&self) -> bool {
        self.years < 0 || self.months < 0 || self.days < 0
    }

    // Checks if all three units of this IsoPeriod are zero
    pub fn is_zero(&self) -> bool {
        self.years == 0 && self.months == 0 && self.days == 0
    }

    // Returns a copy of this IsoPeriod with the specified IsoPeriod subtracted
    pub fn minus(&self, amount_to_subtract: &IsoPeriod) -> Self {
        IsoPeriod {
            years: self.years - amount_to_subtract.years,
            months: self.months - amount_to_subtract.months,
            days: self.days - amount_to_subtract.days,
        }
    }

    // Returns a copy of this IsoPeriod with the specified days subtracted
    pub fn minus_days(&self, days_to_subtract: i32) -> Self {
        IsoPeriod {
            days: self.days - days_to_subtract,
            ..self.clone()
        }
    }

    // Returns a copy of this IsoPeriod with the specified months subtracted
    pub fn minus_months(&self, months_to_subtract: i32) -> Self {
        IsoPeriod {
            months: self.months - months_to_subtract,
            ..self.clone()
        }
    }

    // Returns a copy of this IsoPeriod with the specified years subtracted
    pub fn minus_years(&self, years_to_subtract: i32) -> Self {
        IsoPeriod {
            years: self.years - years_to_subtract,
            ..self.clone()
        }
    }

    // Returns a new instance with each element in this IsoPeriod multiplied by the specified scalar
    pub fn multiplied_by(&self, scalar: i32) -> Self {
        IsoPeriod {
            years: self.years * scalar,
            months: self.months * scalar,
            days: self.days * scalar,
        }
    }

    // Returns a new instance with each amount in this IsoPeriod negated
    pub fn negated(&self) -> Self {
        IsoPeriod {
            years: -self.years,
            months: -self.months,
            days: -self.days,
        }
    }

    // Returns a copy of this IsoPeriod with the years and months normalized
    pub fn normalized(&self) -> Self {
        let total_months = self.years * 12 + self.months;
        let years = total_months / 12;
        let months = total_months % 12;

        IsoPeriod {
            years,
            months,
            days: self.days,
        }
    }

    // Obtains a IsoPeriod representing a number of years, months, and days
    pub fn of(years: i32, months: i32, days: i32) -> Self {
        IsoPeriod { years, months, days }
    }

    // Obtains a IsoPeriod representing a number of days
    pub fn of_days(days: i32) -> Self {
        IsoPeriod { years: 0, months: 0, days }
    }

    // Obtains a IsoPeriod representing a number of months
    pub fn of_months(months: i32) -> Self {
        IsoPeriod { years: 0, months, days: 0 }
    }

    // Obtains a IsoPeriod representing a number of weeks
    pub fn of_weeks(weeks: i32) -> Self {
        IsoPeriod { years: 0, months: 0, days: weeks * 7 }
    }

    // Obtains a IsoPeriod representing a number of years
    pub fn of_years(years: i32) -> Self {
        IsoPeriod { years, months: 0, days: 0 }
    }

    // Obtains a IsoPeriod from a text string such as PnYnMnD
    pub fn parsex(text: &str) -> Option<Self> {
        let re = Regex::new(r"^P(?:(\d+)Y)?(?:(\d+)M)?(?:(\d+)W)?(?:(\d+)D)?$").unwrap();
        if let Some(caps) = re.captures(text) {
            let years = caps.get(1).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
            let months = caps.get(2).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
            let weeks = caps.get(3).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
            let mut days = caps.get(4).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
            days = weeks * 7 + days;

            Some(IsoPeriod { years, months, days })
        } else {
            None
        }
    }
    // pub fn parse(text: &str) -> Option<Self> {
    //     let re = match Regex::new(r"^P(?:(\d+)Y)?(?:(\d+)M)?(?:(\d+)W)?(?:(\d+)D)?$") {
    //         Ok(re) => re,
    //         Err(e) => {
    //             println!("Failed to create regex: {:?}", e);
    //             panic!("Regex compilation failed");
    //         }
    //     };
    //     println!("okok");
    //     let text_upper = text.to_uppercase();
    //     let caps = re.captures(&text_upper)?;
    //
    //     let global_sign = if let Some(sign) = caps.get(1) {
    //         if sign.as_str() == "-" { -1 } else { 1 }
    //     } else {
    //         1
    //     };
    //
    //     let parse_number = |s: Option<&str>| -> Option<i32> {
    //         s.and_then(|s| {
    //             let s = s; //.as_str();
    //             if s.is_empty() {
    //                 Some(0)
    //             } else {
    //                 s.parse::<i32>().ok().map(|value| {
    //                     if s.starts_with('+') || s.starts_with('-') {
    //                         value
    //                     } else {
    //                         value * global_sign
    //                     }
    //                 })
    //             }
    //         })
    //     };
    //
    //     let years = parse_number(caps.get(2).map(|m| m.as_str()))?;
    //     let months = parse_number(caps.get(3).map(|m| m.as_str()))?;
    //     let weeks = parse_number(caps.get(4).map(|m| m.as_str()))?;
    //     let days_initial = parse_number(caps.get(5).map(|m| m.as_str()))?;
    //
    //     // Vérifier qu'au moins une section était présente
    //     let has_any_section = caps.get(2).is_some() || caps.get(3).is_some() || caps.get(4).is_some() || caps.get(5).is_some();
    //     if !has_any_section {
    //         return None;
    //     }
    //
    //     // Vérifier que semaines ne sont pas mélangées avec autres unités
    //     if weeks != 0 && (years != 0 || months != 0 || days_initial != 0) {
    //         return None;
    //     }
    //
    //     let days = days_initial + weeks * 7;
    //
    //     Some(IsoPeriod { years, months, days })
    // }


    // Returns a copy of this IsoPeriod with the specified IsoPeriod added
    pub fn plus(&self, amount_to_add: &IsoPeriod) -> Self {
        IsoPeriod {
            years: self.years + amount_to_add.years,
            months: self.months + amount_to_add.months,
            days: self.days + amount_to_add.days,
        }
    }

    // Returns a copy of this IsoPeriod with the specified days added
    pub fn plus_days(&self, days_to_add: i32) -> Self {
        IsoPeriod {
            days: self.days + days_to_add,
            ..self.clone()
        }
    }

    // Returns a copy of this IsoPeriod with the specified months added
    pub fn plus_months(&self, months_to_add: i32) -> Self {
        IsoPeriod {
            months: self.months + months_to_add,
            ..self.clone()
        }
    }

    // Returns a copy of this IsoPeriod with the specified years added
    pub fn plus_years(&self, years_to_add: i32) -> Self {
        IsoPeriod {
            years: self.years + years_to_add,
            ..self.clone()
        }
    }

    // Subtracts this IsoPeriod from the specified temporal object (NaiveDate in this case)
    pub fn subtract_from(&self, date: NaiveDate) -> NaiveDate {
        date.checked_sub_signed(chrono::Duration::days(self.days as i64)).unwrap()
            .checked_sub_months(chrono::Months::new(self.months as u32)).unwrap()
            .with_year(date.year() - self.years).unwrap()
    }

    // Returns a copy of this IsoPeriod with the specified amount of days
    pub fn with_days(&self, days: i32) -> Self {
        IsoPeriod { days, ..self.clone() }
    }

    // Returns a copy of this IsoPeriod with the specified amount of months
    pub fn with_months(&self, months: i32) -> Self {
        IsoPeriod { months, ..self.clone() }
    }

    // Returns a copy of this IsoPeriod with the specified amount of years
    pub fn with_years(&self, years: i32) -> Self {
        IsoPeriod { years, ..self.clone() }
    }

    // Gets the total number of months in this IsoPeriod
    pub fn to_total_months(&self) -> i32 {
        self.years * 12 + self.months
    }
    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        string_map.get(key).and_then(|s| IsoPeriod::parsex(s.extract_string().unwrap().as_str()))
    }
}


// Implement Display trait for IsoPeriod to enable to_string method
impl fmt::Display for IsoPeriod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "P")?;
        if self.years != 0 {
            write!(f, "{}Y", self.years)?;
        }
        if self.months != 0 {
            write!(f, "{}M", self.months)?;
        }
        if self.days != 0 {
            write!(f, "{}D", self.days)?;
        }
        Ok(())
    }
}


// Implement Hash trait for IsoPeriod to enable hash_code method
impl Hash for IsoPeriod {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.years.hash(state);
        self.months.hash(state);
        self.days.hash(state);
    }
}



impl Add<IsoDatetime> for IsoPeriod {
    type Output = IsoDatetime;

    fn add(self, other: IsoDatetime) -> IsoDatetime {
        // Convert Yards to Meters and add to self
        other.checked_add_days(Days::new(self.days as u64)).unwrap()
            .checked_add_months(Months::new(self.months as u32)).unwrap()
            .checked_add_months(Months::new((self.years * 12)as u32)).unwrap()
    }
}

impl Sub<IsoDatetime> for IsoPeriod {
    type Output = IsoDatetime;

    fn sub(self, other: IsoDatetime) -> IsoDatetime {
        // Convert Yards to Meters and add to self
        other.checked_sub_days(Days::new(self.days as u64)).unwrap()
            .checked_sub_months(Months::new(self.months as u32)).unwrap()
            .checked_sub_months(Months::new((self.years * 12) as u32)).unwrap()
    }
}