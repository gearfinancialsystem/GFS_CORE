use std::fmt;
use std::str::FromStr;
// use crate::traits::TraitMarqueurIsoCycle::TraitMarqueurIsoCycle;
use crate::types::cycle_adjuster::PeriodCycleAdjuster::PeriodCycleAdjuster;
use crate::types::cycle_adjuster::WeekdayCycleAdjuster::WeekdayCycleAdjuster;
use crate::types::IsoPeriod::IsoPeriod;

pub const LONG_STUB: char = '0';
pub const SHORT_STUB: char = '1';


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
            IsoCycle::WeekdayCycleAdjuster(_) => None
        }
    }
    pub fn extract_stub(&self) -> Option<char> {
        match self {
            IsoCycle::PeriodCycleAdjuster(pca) => Some(pca.stub.clone()),
            IsoCycle::WeekdayCycleAdjuster(wca) => Some(wca.stub.clone())
        }
    }
}

// impl TraitMarqueurIsoCycle for IsoCycle {
//     // Get the IsoCycle value
//     fn value(&self) -> IsoCycle {
//         self.clone()
//     }
//     
//     // Set the IsoCycle value
//     fn set_value(&mut self, value: &IsoCycle) {
//         *self = value.clone();
//     }
// }

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

    }
}

// Implémentation du trait Display pour IsoCycle
impl fmt::Display for IsoCycle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IsoCycle::PeriodCycleAdjuster(pca) => write!(f, "PeriodCycleAdjuster({})", pca),
            IsoCycle::WeekdayCycleAdjuster(wca) => write!(f, "WeekdayCycleAdjuster({})", wca),
        }
    }
}