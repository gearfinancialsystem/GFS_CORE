use crate::types::IsoDatetime::IsoDatetime;
use crate::types::IsoPeriod::IsoPeriod;
// use crate::exceptions::String::String;
use crate::traits::TraitCycleAdjuster::TraitCycleAdjuster;
use crate::types::IsoCycle::{LONG_STUB, SHORT_STUB};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PeriodCycleAdjuster {
    pub period: IsoPeriod,
    pub stub: char,
}

impl PeriodCycleAdjuster {
    pub fn new(cycle: String) -> Result<Self, String> {
        let period = Self::parse_period(cycle.clone());
        let stub = Self::parse_stub(cycle);

        match (period, stub) {
            (Ok(val_period), Ok(val_stub)) => {
                
                Ok(PeriodCycleAdjuster { period: val_period, stub: val_stub })
            }
            (Err(e), _) | (_, Err(e)) => {
                Err(e)
            }
        }
    }
    /**
    * Parses a period from a cycle string.
    */
    pub fn parse_period(cycle: String) -> Result<IsoPeriod, String> {
        let period_part = cycle.split('L').next().unwrap();
        match IsoPeriod::parsex(period_part) {
            Some(period) => Ok(period),
            None => Err("te".to_string()),
        }
    }

    /**
    * Parses the stub from the cycle string.
    */
    pub fn parse_stub(cycle: String) -> Result<char, String> {
        let stub_part = cycle.split('L').nth(1).ok_or("te".to_string())?;
        let stub = stub_part.chars().next().ok_or("te".to_string())?;
        if stub == LONG_STUB || stub == SHORT_STUB {
            Ok(stub)
        } else {
            Err("te".to_string())
        }
    }


}

impl TraitCycleAdjuster for PeriodCycleAdjuster {
    fn plus_cycle(&self, time: IsoDatetime) -> IsoDatetime {
        time + self.period.clone()
    }

    fn minus_cycle(&self, time: IsoDatetime) -> IsoDatetime {
        time - self.period.clone()
    }
}


#[cfg(test)]
mod tests_period_cycle_adjuster {
    use std::str::FromStr;
    use crate::types::IsoDatetime::IsoDatetime;
    use super::*;
    #[test]
    fn test_plus_1Ws(){
        let mut p_adjuster = PeriodCycleAdjuster::new("P1WL1".to_string()).unwrap();
        let mut t0 = IsoDatetime::from_str("2016-01-01T00:00:00").unwrap();
        let mut t1 = IsoDatetime::from_str("2016-01-08T00:00:00").unwrap();

        let mut test = p_adjuster.plus_cycle(t0) ;
        assert_eq!(t1, test);
    }

    #[test]
    fn test_minus_1Ws(){
        let mut p_adjuster = PeriodCycleAdjuster::new("P1WL1".to_string()).unwrap();
        let mut t0 = IsoDatetime::from_str("2016-01-01T00:00:00").unwrap();
        let mut t1 = IsoDatetime::from_str("2015-12-25T00:00:00").unwrap();
    
        let mut test = p_adjuster.minus_cycle(t0) ;
        assert_eq!(t1, test);
    }
    #[test]
    fn test_plus_1Ms() {
        let mut p_adjuster = PeriodCycleAdjuster::new("P1ML1".to_string()).unwrap();
        let mut t0 = IsoDatetime::from_str("2016-01-31T00:00:00").unwrap();
        let mut t1 = IsoDatetime::from_str("2016-02-29T00:00:00").unwrap();
    
        let mut test = p_adjuster.plus_cycle(t0) ;
        assert_eq!(t1, test);
    }
    
    #[test]
    fn test_minus_1Ms() {
        let mut p_adjuster = PeriodCycleAdjuster::new("P1ML1".to_string()).unwrap();
        let mut t0 = IsoDatetime::from_str("2016-01-01T00:00:00").unwrap();
        let mut t1 = IsoDatetime::from_str("2015-12-01T00:00:00").unwrap();
    
        let mut test = p_adjuster.minus_cycle(t0) ;
        assert_eq!(t1, test);
    }
    #[test]
    fn test_plus_1Ys(){
        let mut p_adjuster = PeriodCycleAdjuster::new("P1YL1".to_string()).unwrap();
        let mut t0 = IsoDatetime::from_str("2016-01-01T00:00:00").unwrap();
        let mut t1 = IsoDatetime::from_str("2017-01-01T00:00:00").unwrap();
    
        let mut test = p_adjuster.plus_cycle(t0) ;
        assert_eq!(t1, test);
    }
    #[test]
    fn test_minus_1Ys(){
        let mut p_adjuster = PeriodCycleAdjuster::new("P1YL1".to_string()).unwrap();
        let mut t0 = IsoDatetime::from_str("2016-01-01T00:00:00").unwrap();
        let mut t1 = IsoDatetime::from_str("2015-01-01T00:00:00").unwrap();
    
        let mut test = p_adjuster.minus_cycle(t0) ;
        assert_eq!(t1, test);
    }
}