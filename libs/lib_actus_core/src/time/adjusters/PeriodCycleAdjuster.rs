use crate::types::isoDatetime::IsoDatetime;
use crate::types::IsoPeriod::IsoPeriod;
use crate::util::CycleUtils::CycleUtils;
use crate::exceptions::AttributeConversionException::AttributeConversionException;
use crate::traits::TraitCycleAdjuster::TraitCycleAdjuster;

pub struct PeriodCycleAdjuster {
    pub period: IsoPeriod,
    pub stub: char,
}

impl PeriodCycleAdjuster {
    pub fn new(cycle: &String) -> Result<Self, AttributeConversionException> {
        let period = CycleUtils::parse_period(cycle);
        let stub = CycleUtils::parse_stub(cycle);

        match (period, stub) {
            (Ok(val_period), Ok(val_stub)) => {
                println!("Both results are Ok: {} and {}", val_period, val_stub);
                Ok(PeriodCycleAdjuster { period: val_period, stub: val_stub })
            }
            (Err(e), _) | (_, Err(e)) => {
                Err(e)
            }
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
    
    use crate::types::isoDatetime::IsoDatetime;
    use super::*;
    #[test]
    fn test_plus_1Ws(){
        println!("P1WL1");
        let mut p_adjuster = PeriodCycleAdjuster::new(&"P1WL1".to_string()).unwrap();
        let mut t0 = IsoDatetime::parse_from_str("2016-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let mut t1 = IsoDatetime::parse_from_str("2016-01-08T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let mut test = p_adjuster.plus_cycle(t0) ;
        assert_eq!(t1, test);
    }

    #[test]
    fn test_minus_1Ws(){
        let mut p_adjuster = PeriodCycleAdjuster::new(&"P1WL1".to_string()).unwrap();
        let mut t0 = IsoDatetime::parse_from_str("2016-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let mut t1 = IsoDatetime::parse_from_str("2015-12-25T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
    
        let mut test = p_adjuster.minus_cycle(t0) ;
        assert_eq!(t1, test);
    }
    #[test]
    fn test_plus_1Ms() {
        let mut p_adjuster = PeriodCycleAdjuster::new(&"P1ML1".to_string()).unwrap();
        let mut t0 = IsoDatetime::parse_from_str("2016-01-31T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let mut t1 = IsoDatetime::parse_from_str("2016-02-29T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
    
        let mut test = p_adjuster.plus_cycle(t0) ;
        assert_eq!(t1, test);
    }
    
    #[test]
    fn test_minus_1Ms() {
        let mut p_adjuster = PeriodCycleAdjuster::new(&"P1ML1".to_string()).unwrap();
        let mut t0 = IsoDatetime::parse_from_str("2016-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let mut t1 = IsoDatetime::parse_from_str("2015-12-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
    
        let mut test = p_adjuster.minus_cycle(t0) ;
        assert_eq!(t1, test);
    }
    #[test]
    fn test_plus_1Ys(){
        let mut p_adjuster = PeriodCycleAdjuster::new(&"P1YL1".to_string()).unwrap();
        let mut t0 = IsoDatetime::parse_from_str("2016-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let mut t1 = IsoDatetime::parse_from_str("2017-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
    
        let mut test = p_adjuster.plus_cycle(t0) ;
        assert_eq!(t1, test);
    }
    #[test]
    fn test_minus_1Ys(){
        let mut p_adjuster = PeriodCycleAdjuster::new(&"P1YL1".to_string()).unwrap();
        let mut t0 = IsoDatetime::parse_from_str("2016-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let mut t1 = IsoDatetime::parse_from_str("2015-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
    
        let mut test = p_adjuster.minus_cycle(t0) ;
        assert_eq!(t1, test);
    }
}