use std::collections::HashMap;
use std::rc::Rc;


use crate::exceptions::ParseError::ParseError;

// Import des différentes conventions
use crate::terms::grp_calendar::businessday::conventions::Nos::NOS;
use crate::terms::grp_calendar::businessday::conventions::Scf::SCF;
use crate::terms::grp_calendar::businessday::conventions::Scmf::SCMF;
use crate::terms::grp_calendar::businessday::conventions::Csf::CSF;
use crate::terms::grp_calendar::businessday::conventions::Csmf::CSMF;
use crate::terms::grp_calendar::businessday::conventions::Scp::SCP;
use crate::terms::grp_calendar::businessday::conventions::Scmp::SCMP;
use crate::terms::grp_calendar::businessday::conventions::Csp::CSP;
use crate::terms::grp_calendar::businessday::conventions::Csmp::CSMP;

use crate::traits::TraitBusinessDayAdjuster::TraitBusinessDayAdjuster;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::types::isoDatetime::IsoDatetime;
use crate::util::CommonUtils::Value;

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
    pub fn new_NOS(calendar: Rc<Calendar>) -> Self {
        Self::NOS(NOS::new(calendar))
    }

    pub fn new_SCF(calendar: Rc<Calendar>) -> Self {
        Self::SCF(SCF::new(calendar))
    }

    pub fn new_SCMF(calendar: Rc<Calendar>) -> Self {
        Self::SCMF(SCMF::new(calendar))
    }

    pub fn new_CSF(calendar: Rc<Calendar>) -> Self {
        Self::CSF(CSF::new(calendar))
    }

    pub fn new_CSMF(calendar: Rc<Calendar>) -> Self {
        Self::CSMF(CSMF::new(calendar))
    }

    pub fn new_SCP(calendar: Rc<Calendar>) -> Self {
        Self::SCP(SCP::new(calendar))
    }

    pub fn new_SCMP(calendar: Rc<Calendar>) -> Self {
        Self::SCMP(SCMP::new(calendar))
    }

    pub fn new_CSP(calendar: Rc<Calendar>) -> Self {
        Self::CSP(CSP::new(calendar))
    }

    pub fn new_CSMP(calendar: Rc<Calendar>) -> Self {
        Self::CSMP(CSMP::new(calendar))
    }

    pub fn description(&self) -> String {
        match self {
            Self::NOS(v)   => v.type_str(),
            Self::SCF(v)   => v.type_str(),
            Self::SCMF(v)  => v.type_str(),
            Self::CSF(v)   => v.type_str(),
            Self::CSMF(v)  => v.type_str(),
            Self::SCP(v)   => v.type_str(),
            Self::SCMP(v)  => v.type_str(),
            Self::CSP(v)   => v.type_str(),
            Self::CSMP(v)  => v.type_str(),
        }
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
        Self::new_NOS(calendar)
    }

    /// Fonction de parsing qui prend en paramètre le calendrier (boxed)
    pub fn parse(s: &str, calendar: Rc<Calendar>) -> Result<BusinessDayAdjuster, ParseError> {
        match s.to_uppercase().as_str() {
            ""      => Ok(Self::default_with_calendar(calendar)),
            "NOS"   => Ok(Self::new_NOS(calendar)),
            "SCF"   => Ok(Self::new_SCF(calendar)),
            "SCMF"  => Ok(Self::new_SCMF(calendar)),
            "CSF"   => Ok(Self::new_CSF(calendar)),
            "CSMF"  => Ok(Self::new_CSMF(calendar)),
            "SCP"   => Ok(Self::new_SCP(calendar)),
            "SCMP"  => Ok(Self::new_SCMP(calendar)),
            "CSP"   => Ok(Self::new_CSP(calendar)),
            "CSMP"  => Ok(Self::new_CSMP(calendar)),
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s) })
        }
    }    

    pub fn provide_box(string_map: &HashMap<String, String>, key: &str, calendar: Rc<Calendar> ) -> Option<Box<Self>> {
        match string_map.get(key) {
            None => Some(Box::new(Self::default_with_calendar(calendar))),
            Some(s) => {
                match Self::parse(s, calendar) {

                    Ok(bdc) => {
                        println!("{:?}", bdc);
                        Some(Box::new(bdc))

                    },
                    Err(_) => None,
                }
            }
        }
    }
    
    pub fn provide(string_map: &HashMap<String, Value>, key: &str, calendar: Rc<Calendar> ) -> Option<Self> {
        match string_map.get(key) {
            None => Some(Self::default_with_calendar(calendar)),
            Some(s) => {
                match Self::parse(s.extract_string().unwrap().as_str(), calendar) {

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

#[cfg(test)]
mod tests_period_cycle_adjuster {
    use crate::types::isoDatetime::IsoDatetime;
    use super::*;
    use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
    use std::rc::Rc;

    #[test]
    fn test_SAME_NoHolidaysCalendar() {
        // list of unadjusted times
        // let mut adjuster = new BusinessDayAdjuster(BusinessDayConventionEnum.NOS, new NoHolidaysCalendar());
        let calendar = Rc::new(Calendar::new_NC());
        let adjuster = BusinessDayAdjuster::new_NOS(calendar.clone());

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
        // Create a calendar and adjuster for Monday to Friday
        let calendar = Rc::new(Calendar::new_MF());
        let adjuster = BusinessDayAdjuster::new_NOS(calendar.clone());

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
        let calendar = Rc::new(Calendar::new_NC());
        let adjuster = BusinessDayAdjuster::new_SCF(calendar.clone());

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
        let calendar = Rc::new(Calendar::new_MF());
        let adjuster = BusinessDayAdjuster::new_SCF(calendar.clone());

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
        let calendar = Rc::new(Calendar::new_NC());
        let adjuster = BusinessDayAdjuster::new_CSF(calendar.clone());

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
        let calendar = Rc::new(Calendar::new_MF());
        let adjuster = BusinessDayAdjuster::new_CSF(calendar.clone());

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
        let calendar = Rc::new(Calendar::new_NC());
        let adjuster = BusinessDayAdjuster::new_SCMF(calendar.clone());

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
        let calendar = Rc::new(Calendar::new_MF());
        let adjuster = BusinessDayAdjuster::new_SCMF(calendar.clone());

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
        let calendar = Rc::new(Calendar::new_NC());
        let adjuster = BusinessDayAdjuster::new_CSMF(calendar.clone());

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
        let calendar = Rc::new(Calendar::new_MF());
        let adjuster = BusinessDayAdjuster::new_CSMF(calendar.clone());

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
        let calendar = Rc::new(Calendar::new_NC());
        let adjuster = BusinessDayAdjuster::new_SCP(calendar.clone());

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
        let calendar = Rc::new(Calendar::new_MF());
        let adjuster = BusinessDayAdjuster::new_SCP(calendar.clone());

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
        let calendar = Rc::new(Calendar::new_NC());
        let adjuster = BusinessDayAdjuster::new_CSP(calendar.clone());

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
        let calendar = Rc::new(Calendar::new_MF());
        let adjuster = BusinessDayAdjuster::new_CSP(calendar.clone());

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
        let calendar = Rc::new(Calendar::new_NC());
        let adjuster = BusinessDayAdjuster::new_SCMP(calendar.clone());

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
        let calendar = Rc::new(Calendar::new_MF());
        let adjuster = BusinessDayAdjuster::new_SCMP(calendar.clone());

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
        let calendar = Rc::new(Calendar::new_NC());
        let adjuster = BusinessDayAdjuster::new_CSMP(calendar.clone());

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
        let calendar = Rc::new(Calendar::new_MF());
        let adjuster = BusinessDayAdjuster::new_CSMP(calendar.clone());

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