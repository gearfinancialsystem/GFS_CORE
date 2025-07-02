use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;


use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_calendar::businessday::conventions::Nos::NOS;
use crate::terms::grp_calendar::businessday::conventions::Scf::SCF;
use crate::terms::grp_calendar::businessday::conventions::Scmf::SCMF;
use crate::terms::grp_calendar::businessday::conventions::Csf::CSF;
use crate::terms::grp_calendar::businessday::conventions::Csmf::CSMF;
use crate::terms::grp_calendar::businessday::conventions::Scp::SCP;
use crate::terms::grp_calendar::businessday::conventions::Scmp::SCMP;
use crate::terms::grp_calendar::businessday::conventions::Csp::CSP;
use crate::terms::grp_calendar::businessday::conventions::Csmp::CSMP;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util::Value::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BusinessDayAdjuster {
    NOS(NOS),
    SCF(SCF),
    SCMF(SCMF),
    CSF(CSF),
    CSMF(CSMF),
    SCP(SCP),
    SCMP(SCMP),
    CSP(CSP),
    CSMP(CSMP),
}

impl BusinessDayAdjuster {

    pub fn new(element: &str, calendar: Rc<Calendar>) -> Result<Self, ParseError> {
        BusinessDayAdjuster::parse(element, calendar)
    }
    
    pub fn shift_bd(&self, date: &IsoDatetime) -> IsoDatetime {
        match self {
            Self::NOS(v)   => v.shift_bd(date),
            Self::SCF(v)   => v.shift_bd(date),
            Self::SCMF(v)  => v.shift_bd(date),
            Self::CSF(v)   => v.shift_bd(date),
            Self::CSMF(v)  => v.shift_bd(date),
            Self::SCP(v)   => v.shift_bd(date),
            Self::SCMP(v)  => v.shift_bd(date),
            Self::CSP(v)   => v.shift_bd(date),
            Self::CSMP(v)  => v.shift_bd(date),
        }
    }

    pub fn shift_sc(&self, date: &IsoDatetime) -> IsoDatetime { // convention: &dyn TraitBusinessDayAdjuster
        match self {
            Self::NOS(v)   => v.shift_sc(date, &v.bdConvention), // convention
            Self::SCF(v)   => v.shift_sc(date, &v.bdConvention),
            Self::SCMF(v)  => v.shift_sc(date, &v.bdConvention),
            Self::CSF(v)   => v.shift_sc(date, &v.bdConvention),
            Self::CSMF(v)  => v.shift_sc(date, &v.bdConvention),
            Self::SCP(v)   => v.shift_sc(date, &v.bdConvention),
            Self::SCMP(v)  => v.shift_sc(date, &v.bdConvention),
            Self::CSP(v)   => v.shift_sc(date, &v.bdConvention),
            Self::CSMP(v)  => v.shift_sc(date, &v.bdConvention),
        }
    }

    pub fn default_with_calendar(calendar: Rc<Calendar>) -> Self {
        // le calendrier doit forcément pré-exister au BusinessDayAdjuster
        Self::NOS(NOS::new(calendar))
    }

    /// Fonction de parsing qui prend en paramètre le calendrier (boxed)
    pub fn parse(s: &str, calendar: Rc<Calendar>) -> Result<BusinessDayAdjuster, ParseError> {
        match s.to_uppercase().as_str() {
            "NOS"   => Ok(Self::NOS(NOS::new(calendar))),
            "SCF"   => Ok(Self::SCF(SCF::new(calendar))),
            "SCMF"  => Ok(Self::SCMF(SCMF::new(calendar))),
            "CSF"   => Ok(Self::CSF(CSF::new(calendar))),
            "CSMF"  => Ok(Self::CSMF(CSMF::new(calendar))),
            "SCP"   => Ok(Self::SCP(SCP::new(calendar))),
            "SCMP"  => Ok(Self::SCMP(SCMP::new(calendar))),
            "CSP"   => Ok(Self::CSP(CSP::new(calendar))),
            "CSMP"  => Ok(Self::CSMP(CSMP::new(calendar))),
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s) })
        }
    }    

    
    pub fn provide(string_map: &HashMap<String, Value>, key: &str, calendar: Rc<Calendar> ) -> Option<Self> {
        match string_map.get(key) {
            None => Some(Self::default_with_calendar(calendar)),
            Some(s) => {
                match Self::parse(s.as_string().unwrap().as_str(), calendar) {

                    Ok(bdc) => {
                        println!("{:?}", bdc);
                        Some(bdc)

                    },
                    Err(_) => None,
                }
            }
        }
    }
}

impl fmt::Display for BusinessDayAdjuster {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NOS(v)   => write!(f, "BusinessDayAdjuster: {}", v.to_string()),
            Self::SCF(v)   => write!(f, "BusinessDayAdjuster: {}", v.to_string()),
            Self::SCMF(v)  => write!(f, "BusinessDayAdjuster: {}", v.to_string()),
            Self::CSF(v)   => write!(f, "BusinessDayAdjuster: {}", v.to_string()),
            Self::CSMF(v)  => write!(f, "BusinessDayAdjuster: {}", v.to_string()),
            Self::SCP(v)   => write!(f, "BusinessDayAdjuster: {}", v.to_string()),
            Self::SCMP(v)  => write!(f, "BusinessDayAdjuster: {}", v.to_string()),
            Self::CSP(v)   => write!(f, "BusinessDayAdjuster: {}", v.to_string()),
            Self::CSMP(v)  => write!(f, "BusinessDayAdjuster: {}", v.to_string()),
        }
    }
}

#[cfg(test)]
mod tests_period_cycle_adjuster {
    use crate::types::IsoDatetime::IsoDatetime;
    use super::*;
    use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
    use std::rc::Rc;

    #[test]
    fn test_SAME_NoHolidaysCalendar() {
        // list of unadjusted times
        // let mut cycle_adjuster = new BusinessDayAdjuster(BusinessDayConventionEnum.NOS, new NoHolidaysCalendar());
        let calendar = Rc::new(Calendar::new("NC").expect("good cal"));
        let adjuster = BusinessDayAdjuster::new("NOS", calendar.clone()).expect("Adjuster good");

        let mut unadjustedTimes: Vec<IsoDatetime> = vec![];

        unadjustedTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedEventTimes: Vec<IsoDatetime> = vec![];
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedCalcTimes: Vec<IsoDatetime> = vec![];
        expectedCalcTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut shiftedEventTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedEventTimes.push(adjuster.shift_bd(ut)));

        let mut shiftedCalcTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedCalcTimes.push(adjuster.shift_sc(ut)));

        assert_eq!(expectedEventTimes, shiftedEventTimes);
        assert_eq!(expectedCalcTimes, shiftedCalcTimes);
    }

    #[test]
    fn test_SAME_MondayToFridayCalendar() {
        // Create a calendar and cycle_adjuster for Monday to Friday
        let calendar = Rc::new(Calendar::new("MF").expect("good cal"));
        //let cycle_adjuster = BusinessDayAdjuster::new_NOS(calendar.clone());
        let adjuster = BusinessDayAdjuster::new("NOS", calendar.clone()).expect("Adjuster good");
        // List of unadjusted times
        let mut unadjustedTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        // List of expected event times shifted according to the business day convention
        let mut expectedEventTimes: Vec<IsoDatetime> = vec![];
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        // List of expected calculation times shifted according to the business day convention
        let mut expectedCalcTimes: Vec<IsoDatetime> = vec![];
        expectedCalcTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        // Shift times to event times according to the business day convention
        let mut shiftedEventTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedEventTimes.push(adjuster.shift_bd(ut)));

        // Shift times to calculation times according to the business day convention
        let mut shiftedCalcTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedCalcTimes.push(adjuster.shift_sc(ut)));

        // Finally compare unshifted and shifted times
        assert_eq!(expectedEventTimes, shiftedEventTimes);
        assert_eq!(expectedCalcTimes, shiftedCalcTimes);
    }

    #[test]
    fn test_SCF_NoHolidaysCalendar() {
        let calendar = Rc::new(Calendar::new("NC").expect("good cal"));
        let adjuster = BusinessDayAdjuster::new("SCF", calendar.clone()).expect("Adjuster good");


        let mut unadjustedTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedEventTimes: Vec<IsoDatetime> = vec![];
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedCalcTimes: Vec<IsoDatetime> = vec![];
        expectedCalcTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut shiftedEventTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedEventTimes.push(adjuster.shift_bd(ut)));

        let mut shiftedCalcTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedCalcTimes.push(adjuster.shift_sc(ut)));

        assert_eq!(expectedEventTimes, shiftedEventTimes);
        assert_eq!(expectedCalcTimes, shiftedCalcTimes);
    }

    #[test]
    fn test_SCF_MondayToFridayCalendar() {
        let calendar = Rc::new(Calendar::new("MF").expect("good cal"));
        //let cycle_adjuster = BusinessDayAdjuster::new_SCF(calendar.clone());
        let adjuster = BusinessDayAdjuster::new("SCF", calendar.clone()).expect("Adjuster good");
        let mut unadjustedTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedEventTimes: Vec<IsoDatetime> = vec![];
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedCalcTimes: Vec<IsoDatetime> = vec![];
        expectedCalcTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut shiftedEventTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedEventTimes.push(adjuster.shift_bd(ut)));

        let mut shiftedCalcTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedCalcTimes.push(adjuster.shift_sc(ut)));

        assert_eq!(expectedEventTimes, shiftedEventTimes);
        assert_eq!(expectedCalcTimes, shiftedCalcTimes);
    }

    #[test]
    fn test_CSF_NoHolidaysCalendar() {
        let calendar = Rc::new(Calendar::new("NC").expect("good cal"));
        let adjuster = BusinessDayAdjuster::new("CSF", calendar.clone()).expect("Adjuster good");
        let mut unadjustedTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedEventTimes: Vec<IsoDatetime> = vec![];
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedCalcTimes: Vec<IsoDatetime> = vec![];
        expectedCalcTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut shiftedEventTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedEventTimes.push(adjuster.shift_bd(ut)));

        let mut shiftedCalcTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedCalcTimes.push(adjuster.shift_sc(ut)));

        assert_eq!(expectedEventTimes, shiftedEventTimes);
        assert_eq!(expectedCalcTimes, shiftedCalcTimes);
    }

    #[test]
    fn test_CSF_MondayToFridayCalendar() {
        let calendar = Rc::new(Calendar::new("MF").expect("good cal"));

        let adjuster = BusinessDayAdjuster::new("CSF", calendar.clone()).expect("Adjuster good");
        let mut unadjustedTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedEventTimes: Vec<IsoDatetime> = vec![];
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedCalcTimes: Vec<IsoDatetime> = vec![];
        expectedCalcTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut shiftedEventTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedEventTimes.push(adjuster.shift_bd(ut)));

        let mut shiftedCalcTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedCalcTimes.push(adjuster.shift_sc(ut)));

        assert_eq!(expectedEventTimes, shiftedEventTimes);
        assert_eq!(expectedCalcTimes, shiftedCalcTimes);
    }

    #[test]
    fn test_SCMF_NoHolidaysCalendar() {
        let calendar = Rc::new(Calendar::new("NC").expect("good cal"));
        let adjuster = BusinessDayAdjuster::new("SCMF", calendar.clone()).expect("Adjuster good");

        let mut unadjustedTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedEventTimes: Vec<IsoDatetime> = vec![];
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedCalcTimes: Vec<IsoDatetime> = vec![];
        expectedCalcTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut shiftedEventTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedEventTimes.push(adjuster.shift_bd(ut)));

        let mut shiftedCalcTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedCalcTimes.push(adjuster.shift_sc(ut)));

        assert_eq!(expectedEventTimes, shiftedEventTimes);
        assert_eq!(expectedCalcTimes, shiftedCalcTimes);
    }

    #[test]
    fn test_SCMF_MondayToFridayCalendar() {
        let calendar = Rc::new(Calendar::new("MF").expect("good cal"));
        let adjuster = BusinessDayAdjuster::new("SCMF", calendar.clone()).expect("Adjuster good");

        let mut unadjustedTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedEventTimes: Vec<IsoDatetime> = vec![];
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedCalcTimes: Vec<IsoDatetime> = vec![];
        expectedCalcTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut shiftedEventTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedEventTimes.push(adjuster.shift_bd(ut)));

        let mut shiftedCalcTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedCalcTimes.push(adjuster.shift_sc(ut)));

        assert_eq!(expectedEventTimes, shiftedEventTimes);
        assert_eq!(expectedCalcTimes, shiftedCalcTimes);
    }

    #[test]
    fn test_CSMF_NoHolidaysCalendar() {
        let calendar = Rc::new(Calendar::new("NC").expect("good cal"));
        let adjuster = BusinessDayAdjuster::new("CSMF", calendar.clone()).expect("Adjuster good");

        let mut unadjustedTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedEventTimes: Vec<IsoDatetime> = vec![];
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedCalcTimes: Vec<IsoDatetime> = vec![];
        expectedCalcTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut shiftedEventTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedEventTimes.push(adjuster.shift_bd(ut)));

        let mut shiftedCalcTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedCalcTimes.push(adjuster.shift_sc(ut)));

        assert_eq!(expectedEventTimes, shiftedEventTimes);
        assert_eq!(expectedCalcTimes, shiftedCalcTimes);
    }

    #[test]
    fn test_CSMF_MondayToFridayCalendar() {
        let calendar = Rc::new(Calendar::new("MF").expect("good cal"));
        let adjuster = BusinessDayAdjuster::new("CSMF", calendar.clone()).expect("Adjuster good");

        let mut unadjustedTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedEventTimes: Vec<IsoDatetime> = vec![];
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedCalcTimes: Vec<IsoDatetime> = vec![];
        expectedCalcTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut shiftedEventTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedEventTimes.push(adjuster.shift_bd(ut)));

        let mut shiftedCalcTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedCalcTimes.push(adjuster.shift_sc(ut)));

        assert_eq!(expectedEventTimes, shiftedEventTimes);
        assert_eq!(expectedCalcTimes, shiftedCalcTimes);
    }

    #[test]
    fn test_SCP_NoHolidaysCalendar() {
        let calendar = Rc::new(Calendar::new("NC").expect("good cal"));
        let adjuster = BusinessDayAdjuster::new("SCP", calendar.clone()).expect("Adjuster good");

        let mut unadjustedTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedEventTimes: Vec<IsoDatetime> = vec![];
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedCalcTimes: Vec<IsoDatetime> = vec![];
        expectedCalcTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut shiftedEventTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedEventTimes.push(adjuster.shift_bd(ut)));

        let mut shiftedCalcTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedCalcTimes.push(adjuster.shift_sc(ut)));

        assert_eq!(expectedEventTimes, shiftedEventTimes);
        assert_eq!(expectedCalcTimes, shiftedCalcTimes);
    }

    #[test]
    fn test_SCP_MondayToFridayCalendar() {
        let calendar = Rc::new(Calendar::new("MF").expect("good cal"));
        let adjuster = BusinessDayAdjuster::new("SCP", calendar.clone()).expect("Adjuster good");

        let mut unadjustedTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedEventTimes: Vec<IsoDatetime> = vec![];
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedCalcTimes: Vec<IsoDatetime> = vec![];
        expectedCalcTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut shiftedEventTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedEventTimes.push(adjuster.shift_bd(ut)));

        let mut shiftedCalcTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedCalcTimes.push(adjuster.shift_sc(ut)));

        assert_eq!(expectedEventTimes, shiftedEventTimes);
        assert_eq!(expectedCalcTimes, shiftedCalcTimes);
    }

    #[test]
    fn test_CSP_NoHolidaysCalendar() {
        let calendar = Rc::new(Calendar::new("NC").expect("good cal"));
        let adjuster = BusinessDayAdjuster::new("CSP", calendar.clone()).expect("Adjuster good");

        let mut unadjustedTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedEventTimes: Vec<IsoDatetime> = vec![];
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedCalcTimes: Vec<IsoDatetime> = vec![];
        expectedCalcTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut shiftedEventTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedEventTimes.push(adjuster.shift_bd(ut)));

        let mut shiftedCalcTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedCalcTimes.push(adjuster.shift_sc(ut)));

        assert_eq!(expectedEventTimes, shiftedEventTimes);
        assert_eq!(expectedCalcTimes, shiftedCalcTimes);
    }

    #[test]
    fn test_CSP_MondayToFridayCalendar() {
        let calendar = Rc::new(Calendar::new("MF").expect("good cal"));
        let adjuster = BusinessDayAdjuster::new("CSP", calendar.clone()).expect("Adjuster good");

        let mut unadjustedTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedEventTimes: Vec<IsoDatetime> = vec![];
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedCalcTimes: Vec<IsoDatetime> = vec![];
        expectedCalcTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut shiftedEventTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedEventTimes.push(adjuster.shift_bd(ut)));

        let mut shiftedCalcTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedCalcTimes.push(adjuster.shift_sc(ut)));

        assert_eq!(expectedEventTimes, shiftedEventTimes);
        assert_eq!(expectedCalcTimes, shiftedCalcTimes);
    }

    #[test]
    fn test_SCMP_NoHolidaysCalendar() {
        let calendar = Rc::new(Calendar::new("NC").expect("good cal"));
        let adjuster = BusinessDayAdjuster::new("SCMP", calendar.clone()).expect("Adjuster good");

        let mut unadjustedTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedEventTimes: Vec<IsoDatetime> = vec![];
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedCalcTimes: Vec<IsoDatetime> = vec![];
        expectedCalcTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut shiftedEventTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedEventTimes.push(adjuster.shift_bd(ut)));

        let mut shiftedCalcTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedCalcTimes.push(adjuster.shift_sc(ut)));

        assert_eq!(expectedEventTimes, shiftedEventTimes);
        assert_eq!(expectedCalcTimes, shiftedCalcTimes);
    }

    #[test]
    fn test_SCMP_MondayToFridayCalendar() {
        let calendar = Rc::new(Calendar::new("MF").expect("good cal"));
        let adjuster = BusinessDayAdjuster::new("SCMP", calendar.clone()).expect("Adjuster good");

        let mut unadjustedTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedEventTimes: Vec<IsoDatetime> = vec![];
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedCalcTimes: Vec<IsoDatetime> = vec![];
        expectedCalcTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut shiftedEventTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedEventTimes.push(adjuster.shift_bd(ut)));

        let mut shiftedCalcTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedCalcTimes.push(adjuster.shift_sc(ut)));

        assert_eq!(expectedEventTimes, shiftedEventTimes);
        assert_eq!(expectedCalcTimes, shiftedCalcTimes);
    }

    #[test]
    fn test_CSMP_NoHolidaysCalendar() {
        let calendar = Rc::new(Calendar::new("NC").expect("good cal"));
        let adjuster = BusinessDayAdjuster::new("CSMP", calendar.clone()).expect("Adjuster good");

        let mut unadjustedTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedEventTimes: Vec<IsoDatetime> = vec![];
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedCalcTimes: Vec<IsoDatetime> = vec![];
        expectedCalcTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut shiftedEventTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedEventTimes.push(adjuster.shift_bd(ut)));

        let mut shiftedCalcTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedCalcTimes.push(adjuster.shift_sc(ut)));

        assert_eq!(expectedEventTimes, shiftedEventTimes);
        assert_eq!(expectedCalcTimes, shiftedCalcTimes);
    }

    #[test]
    fn test_CSMP_MondayToFridayCalendar() {
        let calendar = Rc::new(Calendar::new("MF").expect("good cal"));
        let adjuster = BusinessDayAdjuster::new("CSMP", calendar.clone()).expect("Adjuster good");

        let mut unadjustedTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        unadjustedTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedEventTimes: Vec<IsoDatetime> = vec![];
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedEventTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut expectedCalcTimes: Vec<IsoDatetime> = vec![];
        expectedCalcTimes.push(IsoDatetime::parse_from_str("29-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("30-04-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("01-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));
        expectedCalcTimes.push(IsoDatetime::parse_from_str("02-05-2016 00:00:00", "%d-%m-%Y %H:%M:%S").expect(""));

        let mut shiftedEventTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedEventTimes.push(adjuster.shift_bd(ut)));

        let mut shiftedCalcTimes: Vec<IsoDatetime> = vec![];
        unadjustedTimes.iter().for_each(|ut| shiftedCalcTimes.push(adjuster.shift_sc(ut)));

        assert_eq!(expectedEventTimes, shiftedEventTimes);
        assert_eq!(expectedCalcTimes, shiftedCalcTimes);
    }
}