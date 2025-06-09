use chrono::Duration;
use humantime::parse_duration;
use chrono::Weekday::{Mon, Tue, Wed, Thu, Fri, Sat, Sun};
use crate::AttributeConversionException::AttributeConversionException;


pub struct CycleUtils;

impl CycleUtils {
    /// A period-based cycle starts with character 'P'
    pub fn is_period(cycle: &str) -> bool {
        cycle.chars().next() == Some('P')
    }

    /// Parses a period from a cycle string
    pub fn parse_period(cycle: &str) -> Result<Duration, AttributeConversionException> {
        // Extract the period string by splitting before the 'L' character
        let period_str = cycle.split('L').next().ok_or(AttributeConversionException)?;
        
        // Parse the period duration (e.g., "P1Y" for one year)
        match parse_duration(period_str) {
            Ok(duration) => Ok(Duration::seconds(duration.as_secs() as i64)),
            Err(_) => Err(AttributeConversionException),
        }
    }

    /// Parses the position as an integer from the first character of the cycle string
    pub fn parse_position(cycle: &str) -> Result<i32, AttributeConversionException> {
        cycle.chars().next()
            .and_then(|c| c.to_digit(10))
            .map(|n| n as i32)
            .ok_or(AttributeConversionException)
    }

    /// Parses a weekday from a cycle string
    pub fn parse_weekday(cycle: &str) -> Result<chrono::Weekday, AttributeConversionException> {
        let weekday_str = &cycle[1..].split('L').next().ok_or(AttributeConversionException)?;
        match weekday_str {
            &"Mon" => Ok(Mon),
            &"Tue" => Ok(Tue),
            &"Wed" => Ok(Wed),
            &"Thu" => Ok(Thu),
            &"Fri" => Ok(Fri),
            &"Sat" => Ok(Sat),
            &"Sun" => Ok(Sun),
            _ => Err(AttributeConversionException),
        }
    }

    /// Parses the stub character from the cycle string
    pub fn parse_stub(cycle: &str) -> Result<char, AttributeConversionException> {
        let stub_str = cycle.split('L').nth(1).ok_or(AttributeConversionException)?;
        let stub = stub_str.chars().next().ok_or(AttributeConversionException)?;
        
        // Assuming LongStub and ShortStub are predefined constants
        if stub == 'L' || stub == 'S' {
            Ok(stub)
        } else {
            Err(AttributeConversionException)
        }
    }

}


