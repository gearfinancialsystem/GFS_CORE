
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;



use crate::terms::grp_calendar::eom_conventions::Eom::EOM;
use crate::terms::grp_calendar::eom_conventions::Sd::SD;
use crate::traits::TraitEndOfMonthConvention::TraitEndOfMonthConvention;


use lib_actus_types::types::IsoCycle::IsoCycle;
use lib_actus_types::types::IsoDatetime::{TraitNaiveDateTimeExtension, IsoDatetime};

use lib_actus_types::types::Value::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum EndOfMonthConvention {
    SD(SD),
    EOM(EOM)
}

impl EndOfMonthConvention {

    pub fn shift(&self, date: IsoDatetime) -> IsoDatetime {
        match self {
            EndOfMonthConvention::SD(SD) => SD.shift(&date),
            EndOfMonthConvention::EOM(EOM) => EOM.shift(&date)
        }
    }

    pub fn new(end_of_month_convention: EndOfMonthConvention, ref_date: IsoDatetime, cycle: IsoCycle) -> Result<Self, String> {
        match end_of_month_convention {
            Self::EOM(EOM) => {
                if ref_date == ref_date.last_date_of_month() &&
                    matches!(cycle, IsoCycle::PeriodCycleAdjuster(_)) { 
                    if cycle.extract_period().unwrap().get_months() > 0 {
                        Ok(EndOfMonthConvention::EOM(EOM))
                    }
                    else { 
                        Ok(EndOfMonthConvention::SD(SD))
                    }
                }
                else {
                    Ok(EndOfMonthConvention::SD(SD))
                }
            },
            Self::SD(SD) => Ok(EndOfMonthConvention::SD(SD)),

        }
    }

    pub fn new_SD() -> Self {
        EndOfMonthConvention::SD(SD::new())
    }

    pub fn new_EOM() -> Self {
        EndOfMonthConvention::EOM(EOM::new())
    }



    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        match string_map.get(key) {
            None => Some(Self::default()), // Clé absente : valeur par défaut dans un Some
            Some(s) => {
                match EndOfMonthConvention::from_str(s.as_string().unwrap().as_str()) {
                    Ok(value) => Some(value), // Valeur valide
                    Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                }
            }
        }
    }
    pub fn provide_from_input_dict(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        match string_map.get(key) {
            None => None,// A VERIFIER // Clé absente : valeur par défaut dans un Some
            Some(s) => {
                match Self::from_str(s.as_string().unwrap().as_str()) {
                    Ok(value) => Some(value), // Valeur valide
                    Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                }
            }
        }
    }
}

impl FromStr for EndOfMonthConvention {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "SD" => Ok(Self::new_SD()),
            "EOM" => Ok(Self::new_EOM()),
            _ => Err(format!("Invalid BusinessDayAdjuster: {}", s))
        }
    }
}

impl Default for EndOfMonthConvention {
    fn default() -> Self {
        Self::new_SD()
    }
}

impl fmt::Display for EndOfMonthConvention {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SD(sd) => write!(f, "{}", sd.to_string()),
            Self::EOM(eom) => write!(f, "{}", eom.to_string()),
        }
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

    // #[test]
    // fn test_sd_start_date_is_not_eom_cycle_m() {
    //     // let calendar = Rc::new(Calendar::new_NC());
    //     let adjuster = EndOfMonthConvention::new(   
    //         EndOfMonthConvention::SD(SD),
    //         parse_date("2016-02-01T00:00:00"),
    //         IsoCycle::from_str("P1ML1").unwrap()).expect("Failed to parse date");
    // 
    // 
    //     let unadjusted_times = vec![
    //         parse_date("2016-04-30T00:00:00"),
    //         parse_date("2016-05-30T00:00:00"),
    //     ];
    // 
    //     let expected_times = vec![
    //         parse_date("2016-04-30T00:00:00"),
    //         parse_date("2016-05-30T00:00:00"),
    //     ];
    // 
    //     let shifted_times: Vec<NaiveDateTime> = unadjusted_times.iter().map(|&t| adjuster.shift(t)).collect();
    // 
    //     assert_eq!(expected_times, shifted_times);
    // }
    // 
    // #[test]
    // fn test_sd_start_date_is_eom_cycle_d() {
    //     let adjuster = EndOfMonthConvention::new(   
    //         EndOfMonthConvention::SD(SD),
    //         parse_date("2016-02-29T00:00:00"),
    //         IsoCycle::from_str("P1DL1").unwrap()).expect("Failed to parse date");
    //     
    //     let unadjusted_times = vec![
    //         parse_date("2016-04-30T00:00:00"),
    //         parse_date("2016-05-30T00:00:00"),
    //     ];
    // 
    //     let expected_times = vec![
    //         parse_date("2016-04-30T00:00:00"),
    //         parse_date("2016-05-30T00:00:00"),
    //     ];
    // 
    //     let shifted_times: Vec<NaiveDateTime> = unadjusted_times.iter().map(|&t| adjuster.shift(t)).collect();
    // 
    //     assert_eq!(expected_times, shifted_times);
    // }
    // 
    // #[test]
    // fn test_sd_start_date_is_eom_cycle_w() {
    //     let adjuster = EndOfMonthConvention::new(   
    //         EndOfMonthConvention::SD(SD),
    //         parse_date("2016-02-29T00:00:00"),
    //         IsoCycle::from_str("P1WL1").unwrap()).expect("Failed to parse date");
    //     
    //     let unadjusted_times = vec![
    //         parse_date("2016-04-30T00:00:00"),
    //         parse_date("2016-05-30T00:00:00"),
    //     ];
    // 
    //     let expected_times = vec![
    //         parse_date("2016-04-30T00:00:00"),
    //         parse_date("2016-05-30T00:00:00"),
    //     ];
    // 
    //     let shifted_times: Vec<NaiveDateTime> = unadjusted_times.iter().map(|&t| adjuster.shift(t)).collect();
    // 
    //     assert_eq!(expected_times, shifted_times);
    // }
    // 
    // #[test]
    // fn test_sd_start_date_is_eom_cycle_m() {
    //     let adjuster = EndOfMonthConvention::new(   
    //         EndOfMonthConvention::SD(SD),
    //         parse_date("2016-02-29T00:00:00"),
    //         IsoCycle::from_str("P1ML1").unwrap()).expect("Failed to parse date");
    //     
    //     let unadjusted_times = vec![
    //         parse_date("2016-04-30T00:00:00"),
    //         parse_date("2016-05-30T00:00:00"),
    //     ];
    // 
    //     let expected_times = vec![
    //         parse_date("2016-04-30T00:00:00"),
    //         parse_date("2016-05-30T00:00:00"),
    //     ];
    // 
    //     let shifted_times: Vec<NaiveDateTime> = unadjusted_times.iter().map(|&t| adjuster.shift(t)).collect();
    // 
    //     assert_eq!(expected_times, shifted_times);
    // }
    // 
    // #[test]
    // fn test_eom_start_date_is_not_eom_cycle_m() {
    //     let adjuster = EndOfMonthConvention::new(   EndOfMonthConvention::EOM(EOM),
    //                                                 parse_date("2016-02-01T00:00:00"),
    //                                                 IsoCycle::from_str("P1ML1").unwrap()).expect("Failed to parse date");
    //     
    //     let unadjusted_times = vec![
    //         parse_date("2016-04-30T00:00:00"),
    //         parse_date("2016-05-30T00:00:00"),
    //     ];
    // 
    //     let expected_times = vec![
    //         parse_date("2016-04-30T00:00:00"),
    //         parse_date("2016-05-30T00:00:00"),
    //     ];
    // 
    //     let shifted_times: Vec<NaiveDateTime> = unadjusted_times.iter().map(|&t| adjuster.shift(t)).collect();
    // 
    //     assert_eq!(expected_times, shifted_times);
    // }
    // 
    // #[test]
    // fn test_eom_start_date_is_eom_cycle_d() {
    // 
    //     let adjuster = EndOfMonthConvention::new(   EndOfMonthConvention::EOM(EOM),
    //                                                 parse_date("2016-02-29T00:00:00"),
    //                                                 IsoCycle::from_str("P1DL1").unwrap()).expect("Failed to parse date");
    //     
    //     let unadjusted_times = vec![
    //         parse_date("2016-04-30T00:00:00"),
    //         parse_date("2016-05-30T00:00:00"),
    //     ];
    // 
    //     let expected_times = vec![
    //         parse_date("2016-04-30T00:00:00"),
    //         parse_date("2016-05-30T00:00:00"),
    //     ];
    // 
    //     let shifted_times: Vec<NaiveDateTime> = unadjusted_times.iter().map(|&t| adjuster.shift(t)).collect();
    // 
    //     assert_eq!(expected_times, shifted_times);
    // }
    // 
    // #[test]
    // fn test_eom_start_date_is_eom_cycle_w() {
    //     let adjuster = EndOfMonthConvention::new(   EndOfMonthConvention::EOM(EOM),
    //                                                 parse_date("2016-02-29T00:00:00"),
    //                                                 IsoCycle::from_str("P1WL1").unwrap()).expect("Failed to parse date");
    //     
    //     let unadjusted_times = vec![
    //         parse_date("2016-04-30T00:00:00"),
    //         parse_date("2016-05-30T00:00:00"),
    //     ];
    // 
    //     let expected_times = vec![
    //         parse_date("2016-04-30T00:00:00"),
    //         parse_date("2016-05-30T00:00:00"),
    //     ];
    // 
    //     let shifted_times: Vec<NaiveDateTime> = unadjusted_times.iter().map(|&t| adjuster.shift(t)).collect();
    // 
    //     assert_eq!(expected_times, shifted_times);
    // }
    // 
    // #[test]
    // fn test_eom_start_date_is_eom_cycle_m() {
    //     let adjuster = EndOfMonthConvention::new(   EndOfMonthConvention::EOM(EOM),
    //                                                 parse_date("2016-02-29T00:00:00"),
    //                                                 IsoCycle::from_str("P1ML1").unwrap()).expect("Failed to parse date");
    //     
    //     let unadjusted_times = vec![
    //         parse_date("2016-04-30T00:00:00"),
    //         parse_date("2016-05-30T00:00:00"),
    //     ];
    // 
    //     let expected_times = vec![
    //         parse_date("2016-04-30T00:00:00"),
    //         parse_date("2016-05-31T00:00:00"),
    //     ];
    // 
    //     let shifted_times: Vec<NaiveDateTime> = unadjusted_times.iter().map(|&t| adjuster.shift(t)).collect();
    // 
    //     assert_eq!(expected_times, shifted_times);
    // }
}