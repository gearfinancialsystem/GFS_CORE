use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Datelike, Timelike, Local, TimeZone, Weekday, Utc};
use crate::period::period::IsoPeriod;

#[derive(Debug)]
pub struct IsoDateTime(pub NaiveDateTime);


impl IsoDateTime {
    // Constructor to create a new LocalDateTime from components
    pub fn new(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> Option<Self> {
        NaiveDate::from_ymd_opt(year, month, day).and_then(|date| {
            NaiveTime::from_hms_opt(hour, minute, second).map(|time| {
                IsoDateTime(NaiveDateTime::new(date, time))
            })
        })
    }
    
    pub fn now() -> Self {
        IsoDateTime(Local::now().naive_local())
    }

    // Static method to get the current date and time in a specific time zone
    fn now_with_zone<Tz: TimeZone>(zone: Tz) -> Self
    where
        Tz::Offset: std::fmt::Display,
    {
        let now = zone.from_utc_datetime(&chrono::Utc::now().naive_utc());
        IsoDateTime(now.naive_local())
    }

    pub fn of_ymd_hms(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> Option<Self> {
        NaiveDate::from_ymd_opt(year, month, day).and_then(|date| {
            NaiveTime::from_hms_opt(hour, minute, second).map(|time| {
                IsoDateTime(NaiveDateTime::new(date, time))
            })
        })
    }

    pub fn of_d_t(date: NaiveDate, time: NaiveTime) -> Self {
        IsoDateTime(NaiveDateTime::new(date, time))
    }

    pub fn parse(text: &str, format: &str) -> Result<Self, chrono::ParseError> {
        let naive_datetime = NaiveDateTime::parse_from_str(text, format);
        naive_datetime.map(IsoDateTime)
    }

    // Function to adjust the given NaiveDateTime to match the date-time of this IsoDateTime
    // addTo(Temporal temporal)
    pub fn adjust_into(&self, other: NaiveDateTime) -> NaiveDateTime {
        let date = self.0.date();
        let time = self.0.time();

        NaiveDateTime::new(
            other.date().with_year(date.year()).unwrap()
                .with_month(date.month()).unwrap()
                .with_day(date.day()).unwrap(),
            other.time().with_hour(time.hour()).unwrap()
                .with_minute(time.minute()).unwrap()
                .with_second(time.second()).unwrap()
        )
    }

    pub fn equals(&self, other: &IsoDateTime) -> bool {
        self.0 == other.0
    }

    // Formats this date-time using the specified formatter.
    pub fn format(&self, formatter: &str) -> String {
        self.0.format(formatter).to_string()
    }

    // Gets the value of the specified field from this date-time as an int.
    // This is a simplified version and may not cover all temporal fields.
    pub fn get(&self, field: &str) -> Option<i32> {
        match field {
            "year" => Some(self.0.year()),
            "month" => Some(self.0.month() as i32),
            "day" => Some(self.0.day() as i32),
            "hour" => Some(self.0.hour() as i32),
            "minute" => Some(self.0.minute() as i32),
            "second" => Some(self.0.second() as i32),
            _ => None,
        }
    }

    // Gets the day-of-month field.
    pub fn get_day_of_month(&self) -> u32 {
        self.0.day()
    }

    // Gets the day-of-week field, which is an enum Weekday in Rust.
    pub fn get_day_of_week(&self) -> Weekday {
        self.0.weekday()
    }

    // Gets the day-of-year field.
    pub fn get_day_of_year(&self) -> u32 {
        self.0.ordinal()
    }

    // Gets the hour-of-day field.
    pub fn get_hour(&self) -> u32 {
        self.0.hour()
    }

    // Gets the value of the specified field from this date-time as a long.
    pub fn get_long(&self, field: &str) -> Option<i64> {
        self.get(field).map(|v| v as i64)
    }

    // Gets the minute-of-hour field.
    pub fn get_minute(&self) -> u32 {
        self.0.minute()
    }

    // Gets the second-of-minute field.
    pub fn get_second(&self) -> u32 {
        self.0.second()
    }

    // Gets the nano-of-second field.
    pub fn get_nano(&self) -> u32 {
        self.0.nanosecond()
    }

    // Gets the year field.
    pub fn get_year(&self) -> i32 {
        self.0.year()
    }

    // Computes a hash code for this date-time.
    pub fn hash_code(&self) -> i64 {
        let year = self.0.year() as i64;
        let month = self.0.month() as i64;
        let day = self.0.day() as i64;
        let hour = self.0.hour() as i64;
        let minute = self.0.minute() as i64;
        let second = self.0.second() as i64;

        year ^ month ^ day ^ hour ^ minute ^ second
    }
    // Checks if this date-time is after the specified date-time.
    pub fn is_after(&self, other: &IsoDateTime) -> bool {
        self.0 > other.0
    }

    // Checks if this date-time is before the specified date-time.
    pub fn is_before(&self, other: &IsoDateTime) -> bool {
        self.0 < other.0
    }

    // Checks if this date-time is equal to the specified date-time.
    pub fn is_equal(&self, other: &IsoDateTime) -> bool {
        self.0 == other.0
    }

    // Checks if the specified field is supported.
    // This is a simplified version and checks only basic fields.
    pub fn is_supported_field(&self, field: &str) -> bool {
        matches!(
            field,
            "year" | "month" | "day" | "hour" | "minute" | "second" | "nanosecond"
        )
    }

    // Checks if the specified unit is supported.
    // This is a simplified version and checks only basic units.
    pub fn is_supported_unit(&self, unit: &str) -> bool {
        matches!(
            unit,
            "years" | "months" | "weeks" | "days" | "hours" | "minutes" | "seconds" | "milliseconds" | "microseconds" | "nanoseconds"
        )
    }

    pub fn to_local_date(&self) -> NaiveDate {
        self.0.date()
    }

    // Gets the LocalTime part of this date-time.
    pub fn to_local_time(&self) -> NaiveTime {
        self.0.time()
    }

    // Outputs this date-time as a String, such as "2007-12-03T10:15:30".
    pub fn to_string(&self) -> String {
        self.0.format("%Y-%m-%dT%H:%M:%S").to_string()
    }
}

impl PartialEq for IsoDateTime {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
