use crate::subtypes::IsoDatetime::IsoDatetime;
use std::collections::HashMap;
use std::rc::Rc;
use crate::terms::grp_calendar::{calendars, Calendar::Calendar};
use crate::traits::BusinessDayCalendarTrait::BusinessDayCalendarTrait;
use crate::traits::DayCountConventionTrait::DayCountConventionTrait;

use chrono::Duration;
use chrono::{Datelike, Timelike};
use std::str::FromStr;
use crate::util::ParseError::ParseError;

use crate::terms::grp_interest::daycountconventions::A336::A336;
use crate::terms::grp_interest::daycountconventions::A360::A360;
use crate::terms::grp_interest::daycountconventions::A365::A365;
use crate::terms::grp_interest::daycountconventions::AAISDA::AAISDA;
use crate::terms::grp_interest::daycountconventions::E283666::E283666;
use crate::terms::grp_interest::daycountconventions::E30360::E30360;
use crate::terms::grp_interest::daycountconventions::B252::B252;
use crate::terms::grp_interest::daycountconventions::E30360ISDA::E30360ISDA;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

#[derive(PartialEq, Eq, Debug)]
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
        DayCountConvention::E30360ISDA(E30360ISDA::new(maturity_date))
    }
    fn new_E30360() -> Self {
        DayCountConvention::E30360(E30360::new())
    }
    fn new_B252(calendar: Rc<dyn BusinessDayCalendarTrait>) -> Self {
        DayCountConvention::B252(B252::new(calendar))
    }
    fn new_E283666(maturity_date: Rc<IsoDatetime>) -> Self {
        DayCountConvention::E283666(E283666::new(maturity_date))
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
        calendar: Rc<dyn BusinessDayCalendarTrait>,
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
                       isodatetime: Rc<IsoDatetime>,
                       calendar_trait:Rc<dyn BusinessDayCalendarTrait> ) -> Option<Box<Self>> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                DayCountConvention::parse(s, isodatetime, calendar_trait).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box
            //.unwrap_or_default()
    }
}

impl Default for DayCountConvention {
    fn default() -> Self {
        DayCountConvention::None
    }
}

impl TermDescriptionTrait for DayCountConvention {
    fn get_identifier(&self) -> &str {
        "dayCountConvention"
    }
    fn get_group(&self) -> &str {
        "Interest"
    }
    fn get_name(&self) -> &str {
        "Day Count Convention"
    }
    fn get_acronym(&self) -> &str {
        "IPDC"
    }
    fn get_type(&self) -> &str {
        "Enum"
    }
    fn get_allowed_values(&self) -> &str {
        "[{'option': '0', 'identifier': 'actualActual', 'name': 'Actual/Actual', 'acronym': 'AA', 'description': 'Year fractions accrue on the basis of the actual number of days per month and per year in the respective period.\r'}, {'option': '1', 'identifier': 'actualThreeSixty', 'name': 'Actual Three Sixty', 'acronym': 'A360', 'description': 'Year fractions accrue on the basis of the actual number of days per month and 360 days per year in the respective period.\r'}, {'option': '2', 'identifier': 'actualThreeSixtyFive', 'name': 'Actual Three Sixty Five', 'acronym': 'A365', 'description': 'Year fractions accrue on the basis of the actual number of days per month and 365 days per year in the respective period.\r'}, {'option': '3', 'identifier': 'thirtyEThreeSixtyISDA', 'name': 'Thirty E Three Sixty ISDA', 'acronym': '30E360ISDA', 'description': 'Year fractions accrue on the basis of 30 days per month and 360 days per year in the respective period (ISDA method).\r'}, {'option': '4', 'identifier': 'thirtyEThreeSixty', 'name': 'Thirty E Three Sixty', 'acronym': '30E360', 'description': 'Year fractions accrue on the basis of 30 days per month and 360 days per year in the respective period.\r'}, {'option': '5', 'identifier': 'twentyEightEThreeThirtySix', 'name': 'Twenty Eight E Three Thirty Six', 'acronym': '28E336', 'description': 'Year fractions accrue on the basis of 28 days per month and 336 days per year in the respective period.'}]"
    }
    fn get_default_value(&self) -> &str {
        ""
    }
    fn get_description(&self) -> &str {
        "Method defining how days are counted between two dates. This finally defines the year fraction in accrual calculations."
    }
}    


