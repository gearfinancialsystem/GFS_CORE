use std::collections::HashMap;
use std::str::FromStr;
use std::rc::Rc;
use crate::subtypes::IsoDatetime::IsoDatetime;

use crate::util::ParseError::ParseError;

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
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

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

    pub fn new_SCF(calendar: Rc<dyn BusinessDayCalendarTrait>) -> Self {
        Self::SCF(SCF::new(calendar))
    }

    pub fn new_SCMF(calendar: Rc<dyn BusinessDayCalendarTrait>) -> Self {
        Self::SCMF(SCMF::new(calendar))
    }

    pub fn new_CSF(calendar: Rc<dyn BusinessDayCalendarTrait>) -> Self {
        Self::CSF(CSF::new(calendar))
    }

    pub fn new_CSMF(calendar: Rc<dyn BusinessDayCalendarTrait>) -> Self {
        Self::CSMF(CSMF::new(calendar))
    }

    pub fn new_SCP(calendar: Rc<dyn BusinessDayCalendarTrait>) -> Self {
        Self::SCP(SCP::new(calendar))
    }

    pub fn new_SCMP(calendar: Rc<dyn BusinessDayCalendarTrait>) -> Self {
        Self::SCMP(SCMP::new(calendar))
    }

    pub fn new_CSP(calendar: Rc<dyn BusinessDayCalendarTrait>) -> Self {
        Self::CSP(CSP::new(calendar))
    }

    pub fn new_CSMP(calendar: Rc<dyn BusinessDayCalendarTrait>) -> Self {
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

    pub fn shift_sc(&self, date: &IsoDatetime, convention: &dyn BusinessDayConventionTrait) -> IsoDatetime {
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
    pub fn parse(s: &str, calendar: Rc<dyn BusinessDayCalendarTrait>) -> Result<BusinessDayConvention, ParseError> {
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

    pub fn provide_box(string_map: &HashMap<String, String>, key: &str, calendar_trait:Rc<dyn BusinessDayCalendarTrait> ) -> Option<Box<Self>> {
        // Exemple 2 : parse de BusinessDayConvention en se basant sur le Calendar.
        // On crée une variable intermédiaire annotée pour convertir le Rc<Calendar>
        // en Rc<dyn BusinessDayCalendarTrait> (puisque Calendar implémente BusinessDayCalendarTrait).
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

impl TermDescriptionTrait for BusinessDayConvention {
    fn get_identifier(&self) -> &str {
        "businessDayConvention"
    }
    fn get_group(&self) -> &str {
        "Calendar"
    }
    fn get_name(&self) -> &str {
        "Business Day Convention"
    }
    fn get_acronym(&self) -> &str {
        "BDC"
    }
    fn get_type(&self) -> &str {
        "Enum"
    }
    fn get_allowed_values(&self) -> &str {
        "[{'option': '0', 'identifier': 'noShift', 'name': 'No Shift', 'acronym': 'NOS', 'description': 'No shift applied to non-business days.\r'}, {'option': '1', 'identifier': 'shiftCalculateFollowing', 'name': 'Shift-Calculate Following', 'acronym': 'SCF', 'description': 'Shift event dates first then calculate accruals etc. Strictly shift to the next following business day.\r'}, {'option': '2', 'identifier': 'shiftCalculateModifiedFollowing', 'name': 'Shift-Calculate Modified-Following', 'acronym': 'SCMF', 'description': 'Shift event dates first then calculate accruals etc. Shift to the next following business day if this falls in the same month. Shift to the most recent preceding business day otherwise.\r'}, {'option': '3', 'identifier': 'calculateShiftFollowing', 'name': 'Calculate-Shift Following', 'acronym': 'CSF', 'description': 'Calculate accruals etc. first then shift event dates. Strictly shift to the next following business day.\r'}, {'option': '4', 'identifier': 'calculateShiftModifiedFollowing', 'name': 'Calculate-Shift Modified-Following', 'acronym': 'CSMF', 'description': 'Calculate accruals etc. first then shift event dates. Shift to the next following business day if this falls in the same month. Shift to the most recent preceding business day otherwise.\r'}, {'option': '5', 'identifier': 'shiftCalculatePreceding', 'name': 'Shift-Calculate Preceding', 'acronym': 'SCP', 'description': 'Shift event dates first then calculate accruals etc. Strictly shift to the most recent preceding business day.\r'}, {'option': '6', 'identifier': 'shiftCalculateModifiedPreceding', 'name': 'Shift-Calculate Modified-Preceding', 'acronym': 'SCMP', 'description': 'Shift event dates first then calculate accruals etc. Shift to the most recent preceding business day if this falls in the same month. Shift to the next following business day otherwise.\r'}, {'option': '7', 'identifier': 'calculateShiftPreceding', 'name': 'Calculate-Shift Preceding', 'acronym': 'CSP', 'description': 'Calculate accruals etc. first then shift event dates. Strictly shift to the most recent preceding business day.\r'}, {'option': '8', 'identifier': 'calculateShiftModifiedPreceding', 'name': 'Calculate-Shift Modified-Preceding', 'acronym': 'SCMP', 'description': 'Calculate accruals etc. first then shift event dates. Shift to the most recent preceding business day if this falls in the same month. Shift to the next following business day otherwise.'}]"
    }
    fn get_default_value(&self) -> &str {
        "nos"
    }
    fn get_description(&self) -> &str {
        "BDC's are linked to a calendar. Calendars have working and non-working days. A BDC value other than N means that cash flows cannot fall on non-working days, they must be shifted to the next business day (following) or the previous on (preceding). These two simple rules get refined twofold: - Following modified (preceding): Same like following (preceding), however if a cash flow gets shifted into a new month, then  it is shifted to preceding (following) business day. - Shift/calculate (SC) and calculate/shift (CS). Accrual, principal, and possibly other calculations are affected by this choice. In the case of SC first the dates are shifted and after the shift cash flows are calculated. In the case of CS it is the other way round. Attention: Does not affect non-cyclical dates such as PRD, MD, TD, IPCED since they can be set to the correct date directly."
    }
}    