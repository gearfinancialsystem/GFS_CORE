j'écris une bibliotheque rust, avant d'implémenter la logique core du programme, je définis beaucoup de types avec leurs traits et implémentation. bref. j'ai un probleme :
Par exemple la structure CSF a deux champs : scConvention & bdConvention. bdConvention est de type Calendar, qui doit être parsé a l'entré du programme, tout comme CSF (un BusinessDayAdjuster).
Du coup on me suggere d'utiliser des lifetimes car Calendar doit vivre au moins aussi longtemps que CSF (car CSF dépend de l'existence de calendar). pour éviter de gérer un lifetime particulier, puis-je dire que calendar a un lifetime static ? et ne plus jamais me soucier de mettre &'static partout ?
on peut aussi utiliser &dyn ou &impl à la place des lifetime.
Peut-on éviter les lifetimes et dyns ??

voici les code : 
code 1 :
use std::str::FromStr;
use crate::traits::TraitBusinessDayAdjuster::TraitBusinessDayAdjuster;
use crate::util::String::String;

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

#[derive(Debug, Eq, PartialEq)]
pub enum BusinessDayAdjuster<'a> {
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

impl<'a> BusinessDayAdjuster<'a> {

    pub fn new_NOS() -> Self {
        BusinessDayAdjuster::NOS(NOS::new())
    }
    pub fn new_SCF(calendar: &Calendar) -> Self {
        BusinessDayAdjuster::SCF(SCF::new(&calendar))
    }
    pub fn new_SCMF(calendar: &Calendar) -> Self {
        BusinessDayAdjuster::SCMF(SCMF::new(&calendar))
    }
    pub fn new_CSF(calendar: &Calendar) -> Self {
        BusinessDayAdjuster::CSF(CSF::new(&calendar))
    }
    pub fn new_CSMF(calendar: &Calendar) -> Self {
        BusinessDayAdjuster::CSMF(CSMF::new(&calendar))
    }
    pub fn new_SCP(calendar: &Calendar) -> Self {
        BusinessDayAdjuster::SCP(SCP::new(&calendar))
    }
    pub fn new_SCMP(calendar: &Calendar) -> Self {
        BusinessDayAdjuster::SCMP(SCMP::new(&calendar))
    }
    pub fn new_CSP(calendar: &Calendar) -> Self {
        BusinessDayAdjuster::CSP(CSP::new(&calendar))
    }
    pub fn new_CSMP(calendar: &Calendar) -> Self {
        BusinessDayAdjuster::CSMP(CSMP::new(&calendar))
    }

    pub fn description(&self) -> String {
        match self {
            BusinessDayAdjuster::NOS(NOS) => NOS.type_str(),
            BusinessDayAdjuster::SCF(SCF) => SCF.type_str(),
            BusinessDayAdjuster::SCMF(SCMF) => SCMF.type_str(),
            BusinessDayAdjuster::CSF(CSF) => CSF.type_str(),
            BusinessDayAdjuster::CSMF(CSMF) => CSMF.type_str(),
            BusinessDayAdjuster::SCP(SCP) => SCP.type_str(),
            BusinessDayAdjuster::SCMP(SCMP) => SCMP.type_str(),
            BusinessDayAdjuster::CSP(CSP) => CSP.type_str(),
            BusinessDayAdjuster::CSMP(CSMP) => CSMP.type_str(),
        }
    }

    pub fn shift_bd(&self, date: &NaiveDateTime) -> NaiveDateTime {
        match self {
            BusinessDayAdjuster::NOS(NOS)     =>   NOS.shift_bd(date),
            BusinessDayAdjuster::SCF(SCF)     =>   SCF.shift_bd(date),
            BusinessDayAdjuster::SCMF(SCMF)  =>   SCMF.shift_bd(date),
            BusinessDayAdjuster::CSF(CSF)     =>   CSF.shift_bd(date),
            BusinessDayAdjuster::CSMF(CSMF)  =>   CSMF.shift_bd(date),
            BusinessDayAdjuster::SCP(SCP)     =>   SCP.shift_bd(date),
            BusinessDayAdjuster::SCMP(SCMP)  =>   SCMP.shift_bd(date),
            BusinessDayAdjuster::CSP(CSP)     =>   CSP.shift_bd(date),
            BusinessDayAdjuster::CSMP(CSMP)  =>   CSMP.shift_bd(date),
        }
    }

    pub fn shift_sc(&self, date: &NaiveDateTime, convention: &dyn TraitBusinessDayAdjuster) -> NaiveDateTime {
        match self {
            BusinessDayAdjuster::NOS(NOS)     =>   NOS.shift_sc(date, convention),
            BusinessDayAdjuster::SCF(SCF)     =>   SCF.shift_sc(date,convention),
            BusinessDayAdjuster::SCMF(SCMF)  =>   SCMF.shift_sc(date, convention),
            BusinessDayAdjuster::CSF(CSF)     =>   CSF.shift_sc(date, convention),
            BusinessDayAdjuster::CSMF(CSMF)  =>   CSMF.shift_sc(date, convention),
            BusinessDayAdjuster::SCP(SCP)     =>   SCP.shift_sc(date, convention),
            BusinessDayAdjuster::SCMP(SCMP)  =>   SCMP.shift_sc(date, convention),
            BusinessDayAdjuster::CSP(CSP)     =>   CSP.shift_sc(date, convention),
            BusinessDayAdjuster::CSMP(CSMP)  =>   CSMP.shift_sc(date, convention),
        }
    }
    pub fn parse ( // a la place de FromStr, car j'ai besoin de plus de parametre
            s: &str,
            calendar: &Calendar,
        ) -> Result<BusinessDayAdjuster, String> {
        match s.to_uppercase().as_str() {
            ""      =>    Ok(BusinessDayAdjuster::default()),
            "NOS"   =>    Ok(BusinessDayAdjuster::new_NOS()),
            "SCF"   =>    Ok(BusinessDayAdjuster::new_SCF(calendar)),
            "SCMF"  =>    Ok(BusinessDayAdjuster::new_SCMF(calendar)),
            "CSF"   =>    Ok(BusinessDayAdjuster::new_CSF(calendar)),
            "CSMF"  =>    Ok(BusinessDayAdjuster::new_CSMF(calendar)),
            "SCP"   =>    Ok(BusinessDayAdjuster::new_SCP(calendar)),
            "SCMP"  =>    Ok(BusinessDayAdjuster::new_SCMP(calendar)),
            "CSP"   =>    Ok(BusinessDayAdjuster::new_CSP(calendar)),
            "CSMP"  =>    Ok(BusinessDayAdjuster::new_CSMP(calendar)),
            _ => Err(String { message: format!("Invalid BusinessDayAdjuster: {}", s)})
        }
    }    
}

impl Default for BusinessDayAdjuster {
    fn default() -> Self {
        BusinessDayAdjuster::new_SCF(&Calendar::default())
    }
}

code 2 :

use chrono::NaiveDateTime;
use crate::terms::grp_calendar::businessday::elements::sc_convention::CalcShift::CalcShift;
use crate::terms::grp_calendar::businessday::elements::bd_convention::Following::Following;
use crate::traits::TraitBusinessDayAdjuster::TraitBusinessDayAdjuster;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::traits::TraitShiftCalcConvention::TraitShiftCalcConvention;

#[derive(Debug, Eq, PartialEq)]
pub struct CSF<'a> {
    pub scConvention: CalcShift,
    pub bdConvention: Following<'a>,
}

impl<'a> CSF<'a> {
    /// Construit un `CSF` en empruntant un `Calendar` (référence).
    /// Il faut donc s’assurer que le `Calendar` vit au moins aussi longtemps
    /// que l'instance de `CSF`.
    pub fn new(calendar: &'a Calendar) -> Self {
        CSF {
            scConvention: CalcShift,
            bdConvention: Following::new(calendar),
        }
    }

    pub fn type_str(&self) -> String {
        "CSF day convention".to_string()
    }
    
    /// Appelle la logique de shift d’après la `scConvention` (CalcShift)
    /// en passant une BDC (trait object) en paramètre.
    pub fn shift_sc(&self, date: &NaiveDateTime, convention: &dyn TraitBusinessDayAdjuster) -> NaiveDateTime {
        self.scConvention.shift(date, convention)
    }

    /// Appelle le shift de la BDC locale (Following).
    pub fn shift_bd(&self, date: &NaiveDateTime) -> NaiveDateTime {
        self.bdConvention.shift(date)
    }
}

code 3 :
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::traits::TraitBusinessDayAdjuster::TraitBusinessDayAdjuster;

use chrono::NaiveDateTime;
use chrono::Duration;

#[derive(Debug, Eq, PartialEq)]
pub struct Following<'a> {
    pub calendar: &'a Calendar,
}

/// Implementation of the Following business day convention
///
/// This convention assumes that if a date falls on a non-business day,
/// it is shifted to the next following business day. Hence, if `d` is the
/// initial date and `d'` the shifted date, we have that:
/// - `d' = d` if `d` is a business day
/// - `d' > d` if `d` is a non-business day

impl<'a> Following<'a> {
    /// Constructeur, prend une référence vers un `Calendar`
    ///
    /// # Arguments
    ///
    /// * `calendar` - Le calendrier à utiliser
    pub fn new(calendar: &'a Calendar) -> Self {
        Following { calendar }
    }
}

impl<'a> TraitBusinessDayAdjuster for Following<'a> {
    /// Décale la date d'entrée si elle tombe un jour non ouvré
    fn shift(&self, date: &NaiveDateTime) -> NaiveDateTime {
        let mut shifted_date = *date;
        // Tant que le jour n'est pas ouvré, on l'incrémente de 1 jour
        while !self.calendar.is_business_day(&shifted_date) {
            shifted_date += Duration::days(1);
        }
        shifted_date
    }
}
code 4 :
use std::str::FromStr;
use chrono::NaiveDateTime;
use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use crate::util::String::String;

use crate::terms::grp_calendar::calendars::NoCalendar::NC;
use crate::terms::grp_calendar::calendars::MondayToFriday::MF;

#[derive(PartialEq, Eq, Debug)]
pub enum Calendar {
    NC(NC),
    MF(MF),
}

impl Calendar {
    /// Décrit l'état actuel de l'enum en appelant `presentation` si nécessaire
    pub fn description(&self) -> String {
        match self {
            Calendar::NC(NC) => NC.type_str(),
            Calendar::MF(MF) => MF.type_str()
        }
    }

    pub fn is_business_day(&self, date: &NaiveDateTime) -> bool {
        match self {
            Calendar::NC(NC) => NC.is_business_day(date),
            Calendar::MF(MF) => MF.is_business_day(date)
        }
    }

    pub fn new_NC() -> Self {
        Calendar::NC(NC::new())
    }

    pub fn new_MF() -> Self {
        Calendar::MF(MF::new())
    }
}

impl FromStr for Calendar {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "" => Ok(Calendar::default()),
            "NC" => Ok(Calendar::new_NC()),
            "MF" => Ok(Calendar::new_MF()),
            _ => Err(String {
                message: format!("Invalid Calendar type: {}", s),
            }),
        }
    }
}

impl Default for Calendar {
    fn default() -> Self {
        Calendar::new_NC()
    }
}

