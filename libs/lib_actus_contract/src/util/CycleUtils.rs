use crate::types::IsoPeriod::IsoPeriod;
// Define a custom error type for attribute conversion errors
use crate::exceptions::AttributeConversionException::AttributeConversionException;
use chrono::Weekday;


// Constants for stub types
pub const LONG_STUB: char = '0';
pub const SHORT_STUB: char = '1';
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
        match IsoPeriod::parsex(period_part) {
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
            "Mon" => Ok(Weekday::Mon),
            "Tue" => Ok(Weekday::Tue),
            "Wed" => Ok(Weekday::Wed),
            "Thu" => Ok(Weekday::Thu),
            "Fri" => Ok(Weekday::Fri),
            "Sat" => Ok(Weekday::Sat),
            "Sun" => Ok(Weekday::Sun),
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