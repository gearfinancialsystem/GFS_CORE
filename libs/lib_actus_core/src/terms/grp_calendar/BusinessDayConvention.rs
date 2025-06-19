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

use crate::traits::TraitBusinessDayConvention::TraitBusinessDayConvention;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::types::isoDatetime::IsoDatetime;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BusinessDayConvention {
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

impl BusinessDayConvention {
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

    pub fn shift_sc(&self, date: &IsoDatetime, convention: &dyn TraitBusinessDayConvention) -> IsoDatetime {
        match self {
            Self::NOS(v)   => v.shift_sc(date, convention),
            Self::SCF(v)   => v.shift_sc(date, convention),
            Self::SCMF(v)  => v.shift_sc(date, convention),
            Self::CSF(v)   => v.shift_sc(date, convention),
            Self::CSMF(v)  => v.shift_sc(date, convention),
            Self::SCP(v)   => v.shift_sc(date, convention),
            Self::SCMP(v)  => v.shift_sc(date, convention),
            Self::CSP(v)   => v.shift_sc(date, convention),
            Self::CSMP(v)  => v.shift_sc(date, convention),
        }
    }

    pub fn default_with_calendar(calendar: Rc<Calendar>) -> Self {
        // le calendrier doit forcément pré-exister au BusinessDayConvention
        Self::new_NOS(calendar)
    }

    /// Fonction de parsing qui prend en paramètre le calendrier (boxed)
    pub fn parse(s: &str, calendar: Rc<Calendar>) -> Result<BusinessDayConvention, ParseError> {
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
            _ => Err(ParseError { message: format!("Invalid BusinessDayConvention: {}", s) })
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
    pub fn provide(string_map: &HashMap<String, String>, key: &str, calendar: Rc<Calendar> ) -> Option<Self> {
        match string_map.get(key) {
            None => Some(Self::default_with_calendar(calendar)),
            Some(s) => {
                match Self::parse(s, calendar) {

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

