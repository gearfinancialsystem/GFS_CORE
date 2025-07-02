use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use crate::terms::grp_calendar::{Calendar::Calendar};
use crate::traits::TraitCountConvention::TraitDayCountConvention;
use crate::types::IsoDatetime::IsoDatetime;

use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_interest::cycle_point_of_interest_payment::B::B;
use crate::terms::grp_interest::cycle_point_of_interest_payment::E::E;
use crate::terms::grp_interest::CyclePointOfInterestPayment::CyclePointOfInterestPayment;
use crate::terms::grp_interest::daycountconventions::A336::A336;
use crate::terms::grp_interest::daycountconventions::A360::A360;
use crate::terms::grp_interest::daycountconventions::A365::A365;
use crate::terms::grp_interest::daycountconventions::AAISDA::AAISDA;
use crate::terms::grp_interest::daycountconventions::E283666::E283666;
use crate::terms::grp_interest::daycountconventions::E30360::E30360;
use crate::terms::grp_interest::daycountconventions::B252::B252;
use crate::terms::grp_interest::daycountconventions::E30360ISDA::E30360ISDA;
use crate::util::Value::Value;

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
    
    pub fn new(element: Option<&str>, calendar: Option<Rc<Calendar>>, maturity_date: Option<Rc<IsoDatetime>>) -> Result<Self, ParseError> {
        match element {

            Some(n) => Self::parse(n, maturity_date, calendar),
            None => Ok(DayCountConvention::None),
        }
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
        maturity_date: Option<Rc<IsoDatetime>>,
        calendar: Option<Rc<Calendar>>,
    ) -> Result<DayCountConvention, ParseError> {
        match s.to_uppercase().as_str() {
            "AAISDA"     => Ok(  DayCountConvention::AAISDA(AAISDA::new())  ),
            "A360"       => Ok(  DayCountConvention::A360(A360::new())  ),
            "A365"       => Ok(  DayCountConvention::A365(A365::new())  ),
            "A336"       => Ok(  DayCountConvention::A336(A336::new())  ),
            "E30360ISDA" => Ok(  DayCountConvention::E30360ISDA(E30360ISDA::new( maturity_date  )   ))  ,
            "E30360"     => Ok(  DayCountConvention::E30360(E30360::new())  ),
            "B252"       => Ok(  DayCountConvention::B252(B252::new( calendar.expect("expect Some maturity for E283666")  )   ))  ,
            "E283666"       => Ok(  DayCountConvention::E283666(E283666::new( maturity_date )))  ,
            _ => Err(ParseError { message: format!("Invalid DayCountConvention: {}", s) }  ),
        }
    }

    pub fn provide(string_map: &HashMap<String, Value>,
                       key: &str,
                       ndt: Option<Rc<IsoDatetime>>,
                       calendar_trait: Option<Rc<Calendar>> ) -> Option<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                DayCountConvention::parse(s.as_string().unwrap().as_str(), ndt, calendar_trait).ok()
            })
            .map(|b| b) // On stocke la convention dans une Box
        //.unwrap_or_default()
    }
    pub fn provide_from_input_dict(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        match string_map.get(key) {
            None => None,// A VERIFIER // Clé absente : valeur par défaut dans un Some
            Some(s) => {
                match Self::from_str(s.as_string().unwrap().as_str()) {
                    Ok(value) => Some(value), // Valeur valide
                    Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                }
            }
        }
    }
}

impl Default for DayCountConvention {
    fn default() -> Self {
        DayCountConvention::None
    }
}


impl fmt::Display for DayCountConvention {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AAISDA(AAISDA) => write!(f, "DayCountConvention: {}", AAISDA.to_string()),
            Self::A360(A360) => write!(f, "DayCountConvention: {}", A360.to_string()),
            Self::A365(A365) => write!(f, "DayCountConvention: {}", A365.to_string()),
            Self::A336(A336) => write!(f, "DayCountConvention: {}", A336.to_string()),
            Self::E30360ISDA(E30360ISDA) => write!(f, "DayCountConvention: {}", E30360ISDA.to_string()),
            Self::E30360(E30360) => write!(f, "DayCountConvention: {}", E30360.to_string()),
            Self::B252(B252) => write!(f, "DayCountConvention: {}", B252.to_string()),
            Self::E283666(E283666) => write!(f, "DayCountConvention: {}", E283666.to_string()),
            Self::None => write!(f, "DayCountConvention: None"),
        
        }
    }
}

