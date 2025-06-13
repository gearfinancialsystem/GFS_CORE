use std::collections::HashMap;
use std::str::FromStr;
use std::rc::Rc;
use chrono::NaiveDateTime;

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

use crate::terms::grp_calendar::Calendar::Calendar;
use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use crate::traits::TraitBusinessDayConvention::TraitBusinessDayConvention;
use crate::traits::TraitTermDescription::TraitTermDescription;

#[derive(Debug, Eq, PartialEq)]
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
    pub fn new_NOS() -> Self {
        BusinessDayConvention::NOS(NOS::new())
    }

    pub fn new_SCF(calendar: Rc<dyn TraitBusinessDayCalendar>) -> Self {
        Self::SCF(SCF::new(calendar))
    }

    pub fn new_SCMF(calendar: Rc<dyn TraitBusinessDayCalendar>) -> Self {
        Self::SCMF(SCMF::new(calendar))
    }

    pub fn new_CSF(calendar: Rc<dyn TraitBusinessDayCalendar>) -> Self {
        Self::CSF(CSF::new(calendar))
    }

    pub fn new_CSMF(calendar: Rc<dyn TraitBusinessDayCalendar>) -> Self {
        Self::CSMF(CSMF::new(calendar))
    }

    pub fn new_SCP(calendar: Rc<dyn TraitBusinessDayCalendar>) -> Self {
        Self::SCP(SCP::new(calendar))
    }

    pub fn new_SCMP(calendar: Rc<dyn TraitBusinessDayCalendar>) -> Self {
        Self::SCMP(SCMP::new(calendar))
    }

    pub fn new_CSP(calendar: Rc<dyn TraitBusinessDayCalendar>) -> Self {
        Self::CSP(CSP::new(calendar))
    }

    pub fn new_CSMP(calendar: Rc<dyn TraitBusinessDayCalendar>) -> Self {
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

    pub fn shift_bd(&self, date: &NaiveDateTime) -> NaiveDateTime {
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

    pub fn shift_sc(&self, date: &NaiveDateTime, convention: &dyn TraitBusinessDayConvention) -> NaiveDateTime {
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
    
    /// Fonction de parsing qui prend en paramètre le calendrier (boxed)
    pub fn parse(s: &str, calendar: Rc<dyn TraitBusinessDayCalendar>) -> Result<BusinessDayConvention, ParseError> {
        match s.to_uppercase().as_str() {
            ""      => Ok(Self::default()),
            "NOS"   => Ok(Self::new_NOS()),
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

    pub fn provide_box(string_map: &HashMap<String, String>, key: &str, calendar_trait:Rc<dyn TraitBusinessDayCalendar> ) -> Option<Box<Self>> {
        // Exemple 2 : parse de BusinessDayConvention en se basant sur le Calendar.
        // On crée une variable intermédiaire annotée pour convertir le Rc<Calendar>
        // en Rc<dyn TraitBusinessDayCalendar> (puisque Calendar implémente TraitBusinessDayCalendar).
        string_map
            .get(key)
            .and_then(|s| {
                Self::parse(s, calendar_trait).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box
            //.unwrap_or_default()
    }

}

impl Default for BusinessDayConvention {
    fn default() -> Self {
        // Par défaut, on utilise un calendrier NC.
        Self::new_NOS()
    }
}

