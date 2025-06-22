use crate::exceptions::ParseError::ParseError;
use std::collections::HashMap;
use std::str::FromStr;
use crate::exceptions::AttributeConversionException::AttributeConversionException;
use crate::terms::grp_calendar::eom_conventions::Eom::EOM;
use crate::terms::grp_calendar::eom_conventions::Sd::SD;
use crate::traits::TraitEndOfMonthConvention::TraitEndOfMonthConvention;
use crate::types::isoDatetime::{traitNaiveDateTimeExtension, IsoDatetime};
use crate::util::CycleUtils::CycleUtils;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum EndOfMonthConvention {
    SD(SD),
    EOM(EOM)
}

impl EndOfMonthConvention {
    pub fn description(&self) -> String {
        match self {
            EndOfMonthConvention::SD(SD) => SD.type_str(),
            EndOfMonthConvention::EOM(EOM) => EOM.type_str()
        }
    }

    pub fn shift(&self, date: IsoDatetime) -> IsoDatetime {
        match self {
            EndOfMonthConvention::SD(SD) => SD.shift(&date),
            EndOfMonthConvention::EOM(EOM) => EOM.shift(&date)
        }
    }

    pub fn new(end_of_month_convention: EndOfMonthConvention, ref_date: IsoDatetime, cycle: String) -> Result<Self, AttributeConversionException> {
        match end_of_month_convention {
            Self::EOM(EOM) => {
                if (ref_date == ref_date.last_date_of_month() && CycleUtils::parse_period(&cycle).unwrap().get_months() > 0){ //ok
                    Ok(EndOfMonthConvention::EOM(EOM))
                }
                else {
                    Ok(EndOfMonthConvention::SD(SD))
                }
            },
            Self::SD(SD) => Ok(EndOfMonthConvention::SD(SD)),
            _ => Err(AttributeConversionException) // a virer ?
        }
    }

    pub fn new_SD() -> Self {
        EndOfMonthConvention::SD(SD::new())
    }

    pub fn new_EOM() -> Self {
        EndOfMonthConvention::EOM(EOM::new())
    }

    pub fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Option<Box<Self>> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                Self::from_str(s).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box
            //.unwrap_or_default()
    }

    pub fn provide(string_map: &HashMap<String, String>, key: &str) -> Option<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                Self::from_str(s).ok()
            })
            .map(|b| b) // On stocke la convention dans une Box
        //.unwrap_or_default()
    }
}

impl FromStr for EndOfMonthConvention {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "SD" => Ok(Self::new_SD()),
            "EOM" => Ok(Self::new_EOM()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s)})
        }
    }
}

impl Default for EndOfMonthConvention {
    fn default() -> Self {
        Self::new_SD()
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use super::*;
    use chrono::NaiveDateTime;
    use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
    use crate::terms::grp_calendar::Calendar::Calendar;

    fn parse_date(date_str: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S").expect("Failed to parse date")
    }

    #[test]
    fn test_sd_start_date_is_not_eom_cycle_m() {
        // let calendar = Rc::new(Calendar::new_NC());
        let adjuster = EndOfMonthConvention::new(   EndOfMonthConvention::SD(SD),
                                                             parse_date("2016-02-01T00:00:00"),
                                                        "P1ML1".to_string()).expect("Failed to parse date");


        let unadjusted_times = vec![
            parse_date("2016-04-30T00:00:00"),
            parse_date("2016-05-30T00:00:00"),
        ];

        let expected_times = vec![
            parse_date("2016-04-30T00:00:00"),
            parse_date("2016-05-30T00:00:00"),
        ];

        let shifted_times: Vec<NaiveDateTime> = unadjusted_times.iter().map(|&t| adjuster.shift(t)).collect();

        assert_eq!(expected_times, shifted_times);
    }

    #[test]
    fn test_sd_start_date_is_eom_cycle_d() {
        let adjuster = EndOfMonthConvention::new(   EndOfMonthConvention::SD(SD),
                                                    parse_date("2016-02-29T00:00:00"),
                                                    "P1DL1".to_string()).expect("Failed to parse date");

        let unadjusted_times = vec![
            parse_date("2016-04-30T00:00:00"),
            parse_date("2016-05-30T00:00:00"),
        ];

        let expected_times = vec![
            parse_date("2016-04-30T00:00:00"),
            parse_date("2016-05-30T00:00:00"),
        ];

        let shifted_times: Vec<NaiveDateTime> = unadjusted_times.iter().map(|&t| adjuster.shift(t)).collect();

        assert_eq!(expected_times, shifted_times);
    }

    #[test]
    fn test_sd_start_date_is_eom_cycle_w() {
        let adjuster = EndOfMonthConvention::new(   EndOfMonthConvention::SD(SD),
                                                    parse_date("2016-02-29T00:00:00"),
                                                    "P1WL1".to_string()).expect("Failed to parse date");

        let unadjusted_times = vec![
            parse_date("2016-04-30T00:00:00"),
            parse_date("2016-05-30T00:00:00"),
        ];

        let expected_times = vec![
            parse_date("2016-04-30T00:00:00"),
            parse_date("2016-05-30T00:00:00"),
        ];

        let shifted_times: Vec<NaiveDateTime> = unadjusted_times.iter().map(|&t| adjuster.shift(t)).collect();

        assert_eq!(expected_times, shifted_times);
    }

    #[test]
    fn test_sd_start_date_is_eom_cycle_m() {
        let adjuster = EndOfMonthConvention::new(   EndOfMonthConvention::SD(SD),
                                                    parse_date("2016-02-29T00:00:00"),
                                                    "P1ML1".to_string()).expect("Failed to parse date");

        let unadjusted_times = vec![
            parse_date("2016-04-30T00:00:00"),
            parse_date("2016-05-30T00:00:00"),
        ];

        let expected_times = vec![
            parse_date("2016-04-30T00:00:00"),
            parse_date("2016-05-30T00:00:00"),
        ];

        let shifted_times: Vec<NaiveDateTime> = unadjusted_times.iter().map(|&t| adjuster.shift(t)).collect();

        assert_eq!(expected_times, shifted_times);
    }

    #[test]
    fn test_eom_start_date_is_not_eom_cycle_m() {
        let adjuster = EndOfMonthConvention::new(   EndOfMonthConvention::EOM(EOM),
                                                    parse_date("2016-02-01T00:00:00"),
                                                    "P1ML1".to_string()).expect("Failed to parse date");

        let unadjusted_times = vec![
            parse_date("2016-04-30T00:00:00"),
            parse_date("2016-05-30T00:00:00"),
        ];

        let expected_times = vec![
            parse_date("2016-04-30T00:00:00"),
            parse_date("2016-05-30T00:00:00"),
        ];

        let shifted_times: Vec<NaiveDateTime> = unadjusted_times.iter().map(|&t| adjuster.shift(t)).collect();

        assert_eq!(expected_times, shifted_times);
    }

    #[test]
    fn test_eom_start_date_is_eom_cycle_d() {

        let adjuster = EndOfMonthConvention::new(   EndOfMonthConvention::EOM(EOM),
                                                    parse_date("2016-02-29T00:00:00"),
                                                    "P1DL1".to_string()).expect("Failed to parse date");
        let unadjusted_times = vec![
            parse_date("2016-04-30T00:00:00"),
            parse_date("2016-05-30T00:00:00"),
        ];

        let expected_times = vec![
            parse_date("2016-04-30T00:00:00"),
            parse_date("2016-05-30T00:00:00"),
        ];

        let shifted_times: Vec<NaiveDateTime> = unadjusted_times.iter().map(|&t| adjuster.shift(t)).collect();

        assert_eq!(expected_times, shifted_times);
    }

    #[test]
    fn test_eom_start_date_is_eom_cycle_w() {
        let adjuster = EndOfMonthConvention::new(   EndOfMonthConvention::EOM(EOM),
                                                    parse_date("2016-02-29T00:00:00"),
                                                    "P1WL1".to_string()).expect("Failed to parse date");

        let unadjusted_times = vec![
            parse_date("2016-04-30T00:00:00"),
            parse_date("2016-05-30T00:00:00"),
        ];

        let expected_times = vec![
            parse_date("2016-04-30T00:00:00"),
            parse_date("2016-05-30T00:00:00"),
        ];

        let shifted_times: Vec<NaiveDateTime> = unadjusted_times.iter().map(|&t| adjuster.shift(t)).collect();

        assert_eq!(expected_times, shifted_times);
    }

    #[test]
    fn test_eom_start_date_is_eom_cycle_m() {
        let adjuster = EndOfMonthConvention::new(   EndOfMonthConvention::EOM(EOM),
                                                    parse_date("2016-02-29T00:00:00"),
                                                    "P1ML1".to_string()).expect("Failed to parse date");

        let unadjusted_times = vec![
            parse_date("2016-04-30T00:00:00"),
            parse_date("2016-05-30T00:00:00"),
        ];

        let expected_times = vec![
            parse_date("2016-04-30T00:00:00"),
            parse_date("2016-05-31T00:00:00"),
        ];

        let shifted_times: Vec<NaiveDateTime> = unadjusted_times.iter().map(|&t| adjuster.shift(t)).collect();

        assert_eq!(expected_times, shifted_times);
    }
}