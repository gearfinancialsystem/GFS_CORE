use std::collections::HashMap;
use std::rc::Rc;
use crate::terms::grp_calendar::{Calendar::Calendar};
use crate::traits::TraitCountConvention::TraitDayCountConvention;
use crate::types::isoDatetime::IsoDatetime;

use crate::exceptions::ParseError::ParseError;

use crate::terms::grp_interest::daycountconventions::A336::A336;
use crate::terms::grp_interest::daycountconventions::A360::A360;
use crate::terms::grp_interest::daycountconventions::A365::A365;
use crate::terms::grp_interest::daycountconventions::AAISDA::AAISDA;
use crate::terms::grp_interest::daycountconventions::E283666::E283666;
use crate::terms::grp_interest::daycountconventions::E30360::E30360;
use crate::terms::grp_interest::daycountconventions::B252::B252;
use crate::terms::grp_interest::daycountconventions::E30360ISDA::E30360ISDA;
use crate::util::CommonUtils::Value;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DayCountConvention {
    AAISDA(AAISDA),
    A360(A360),
    A365(A365),
    A336(A336),
    E30360ISDA(E30360ISDA),
    E30360(E30360),
    B252(B252),
    E283666(E283666),
    None
}

impl DayCountConvention {
    fn new_AAISDA() -> Self {
        DayCountConvention::AAISDA(AAISDA::new())
    }
    fn new_A360() -> Self {
        DayCountConvention::A360(A360::new())
    }
    fn new_A365() -> Self {
        DayCountConvention::A365(A365::new())
    }
    fn new_A336() -> Self {
        DayCountConvention::A336(A336::new())
    }
    fn new_E30360ISDA(maturity_date: Rc<IsoDatetime>) -> Self {
        DayCountConvention::E30360ISDA(E30360ISDA::new(Some(maturity_date)))
    }
    fn new_E30360() -> Self {
        DayCountConvention::E30360(E30360::new())
    }
    fn new_B252(calendar: Rc<Calendar>) -> Self {
        DayCountConvention::B252(B252::new(calendar))
    }
    fn new_E283666(maturity_date: Rc<IsoDatetime>) -> Self {
        DayCountConvention::E283666(E283666::new(Some(maturity_date)))
    }

    pub fn day_count(&self,start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        match self {
            DayCountConvention::AAISDA(AAISDA) => AAISDA.day_count(start_time, end_time),
            DayCountConvention::A360(A360) => A360.day_count(start_time, end_time),
            DayCountConvention::A365(A365) => A365.day_count(start_time, end_time),
            DayCountConvention::A336(A336) => A336.day_count(start_time, end_time),
            DayCountConvention::E30360ISDA(E30360ISDA) => E30360ISDA.day_count(start_time, end_time),
            DayCountConvention::E30360(E30360) => E30360.day_count(start_time, end_time),
            DayCountConvention::B252(B252) => B252.day_count(start_time, end_time),
            DayCountConvention::E283666(E283666) => E283666.day_count(start_time, end_time),
            DayCountConvention::None => 0.0,
        }
    }

    pub fn day_count_fraction(&self,start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        match self {
            DayCountConvention::AAISDA(AAISDA) => AAISDA.day_count_fraction(start_time, end_time),
            DayCountConvention::A360(A360) => A360.day_count_fraction(start_time, end_time),
            DayCountConvention::A365(A365) => A365.day_count_fraction(start_time, end_time),
            DayCountConvention::A336(A336) => A336.day_count_fraction(start_time, end_time),
            DayCountConvention::E30360ISDA(E30360ISDA) => E30360ISDA.day_count_fraction(start_time, end_time),
            DayCountConvention::E30360(E30360) => E30360.day_count_fraction(start_time, end_time),
            DayCountConvention::B252(B252) => B252.day_count_fraction(start_time, end_time),
            DayCountConvention::E283666(E283666) => E283666.day_count_fraction(start_time, end_time),
            DayCountConvention::None => 0.0,
        }
    }

    pub fn parse ( // a la place de FromStr, car j'ai besoin de plus de parametre
        s: &str,
        maturity_date: Rc<IsoDatetime>,
        calendar: Rc<Calendar>,
    ) -> Result<DayCountConvention, ParseError> {
        match s.to_uppercase().as_str() {
            "AAISDA"     => Ok(DayCountConvention::new_AAISDA()),
            "A360"       => Ok(DayCountConvention::new_A360()),
            "A365"       => Ok(DayCountConvention::new_A365()),
            "A336"       => Ok(DayCountConvention::new_A336()),
            "E30360ISDA" => Ok(DayCountConvention::new_E30360ISDA(maturity_date)),
            "E30360"     => Ok(DayCountConvention::new_E30360()),
            "B252"       => Ok(DayCountConvention::new_B252(calendar)),
            "E283666"    => Ok(DayCountConvention::new_E283666(maturity_date)),
            _ => Err(ParseError {
                message: format!("Invalid DayCountConvention: {}", s),
            }),
        }
    }
    pub fn provide_box(string_map: &HashMap<String, String>,
                       key: &str,
                       ndt: Rc<IsoDatetime>,
                       calendar_trait:Rc<Calendar> ) -> Option<Box<Self>> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                DayCountConvention::parse(s, ndt, calendar_trait).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box
            //.unwrap_or_default()
    }
    pub fn provide(string_map: &HashMap<String, Value>,
                       key: &str,
                       ndt: Rc<IsoDatetime>,
                       calendar_trait:Rc<Calendar> ) -> Option<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                DayCountConvention::parse(s.extract_string().unwrap().as_str(), ndt, calendar_trait).ok()
            })
            .map(|b| b) // On stocke la convention dans une Box
        //.unwrap_or_default()
    }
}

impl Default for DayCountConvention {
    fn default() -> Self {
        DayCountConvention::None
    }
}



