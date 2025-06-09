use chrono::{NaiveDate, Weekday};
use std::error::Error;
use std::fmt;
use std::str::FromStr;
use super::super::period::period::IsoPeriod;
// Define a custom error type for attribute conversion errors
#[derive(Debug)]
pub struct AttributeConversionException;

impl fmt::Display for AttributeConversionException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Attribute conversion error")
    }
}

impl Error for AttributeConversionException {}

// Constants for stub types
const LONG_STUB: char = '0';
const SHORT_STUB: char = '1';

pub struct CycleUtils;

impl CycleUtils {
    /**
     * Checks if a cycle string starts with 'P', indicating a period-based cycle.
     */
    pub fn is_period(cycle: &String) -> bool {
        cycle.starts_with('P')
    }

    /**
     * Parses a period from a cycle string.
     */
    pub fn parse_period(cycle: &String) -> Result<IsoPeriod, AttributeConversionException> {
        let period_part = cycle.split('L').next().unwrap();
        match IsoPeriod::parse(period_part) {
            Some(period) => Ok(period),
            None => Err(AttributeConversionException),
        }
    }

    /**
     * Parses the position from the cycle string.
     */
    pub fn parse_position(cycle: &String) -> Result<u32, AttributeConversionException> {
        let position_char = cycle.chars().next().ok_or(AttributeConversionException)?;
        position_char.to_digit(10).ok_or(AttributeConversionException)
    }

    /**
     * Parses the weekday from the cycle string.
     */
    pub fn parse_weekday(cycle: &String) -> Result<Weekday, AttributeConversionException> {
        let weekday_part = cycle.split('L').next().unwrap();
        let weekday_str = &weekday_part[1..4]; // Assuming the format is like "1MONL"
        match weekday_str {
            "MON" => Ok(Weekday::Mon),
            "TUE" => Ok(Weekday::Tue),
            "WED" => Ok(Weekday::Wed),
            "THU" => Ok(Weekday::Thu),
            "FRI" => Ok(Weekday::Fri),
            "SAT" => Ok(Weekday::Sat),
            "SUN" => Ok(Weekday::Sun),
            _ => Err(AttributeConversionException),
        }
    }

    /**
     * Parses the stub from the cycle string.
     */
    pub fn parse_stub(cycle: &String) -> Result<char, AttributeConversionException> {
        let stub_part = cycle.split('L').nth(1).ok_or(AttributeConversionException)?;
        let stub = stub_part.chars().next().ok_or(AttributeConversionException)?;
        if stub == LONG_STUB || stub == SHORT_STUB {
            Ok(stub)
        } else {
            Err(AttributeConversionException)
        }
    }
}


