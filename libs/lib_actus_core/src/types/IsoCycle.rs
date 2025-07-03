

use std::str::FromStr;
use crate::types::cycle_adjuster::PeriodCycleAdjuster::PeriodCycleAdjuster;
use crate::types::cycle_adjuster::WeekdayCycleAdjuster::WeekdayCycleAdjuster;
use crate::types::IsoPeriod::IsoPeriod;

pub const LONG_STUB: char = '0';
pub const SHORT_STUB: char = '1';


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IsoCycle {
    PeriodCycleAdjuster(PeriodCycleAdjuster),
    WeekdayCycleAdjuster(WeekdayCycleAdjuster),
}

impl IsoCycle {
    pub fn new(cycle: String) -> Result<Self, String> {
        IsoCycle::from_str(cycle.as_str())
    }
    /**
    * Checks if a cycle string starts with 'P', indicating a period-based cycle.
    */
    pub fn is_period(cycle: &String) -> bool {
        cycle.starts_with('P')
    }
    pub fn extract_period(&self) -> Option<IsoPeriod> {
        match self {
            IsoCycle::PeriodCycleAdjuster(pca) => Some(pca.period.clone()),
            IsoCycle::WeekdayCycleAdjuster(wca) => None
        }
    }
    pub fn extract_stub(&self) -> Option<char> {
        match self {
            IsoCycle::PeriodCycleAdjuster(pca) => Some(pca.stub.clone()),
            IsoCycle::WeekdayCycleAdjuster(wca) => Some(wca.stub.clone())
        }
    }
}

impl FromStr for IsoCycle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_period(&s.to_string()) == true {
            let p_adjuster = PeriodCycleAdjuster::new(s.to_string());
            match p_adjuster {
                Ok(v) => Ok(IsoCycle::PeriodCycleAdjuster(v)),
                Err(_) => Err("Invalid cycle cycle_adjuster: ".to_owned()),
            }
        }
        else {
            let wd_adjuster = WeekdayCycleAdjuster::new(s.to_string()).expect("weekday cycle_adjuster good");
            Ok(IsoCycle::WeekdayCycleAdjuster(   wd_adjuster    ))
        }
        // Err(_) => Err(format!("Unable to parse {} as f64", s)),
    }
}