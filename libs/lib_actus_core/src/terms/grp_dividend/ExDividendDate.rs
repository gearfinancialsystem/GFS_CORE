use crate::types::IsoDatetime::IsoDatetime;
use crate::types::IsoPeriod::IsoPeriod;
use chrono::NaiveDateTime;
use chrono::ParseResult;

use crate::define_struct_isodatetime;
define_struct_isodatetime!(ExDividendDate);