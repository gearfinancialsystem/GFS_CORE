use std::ops::{Add, Sub};
use crate::types::IsoPeriod::IsoPeriod;
use crate::types::IsoDatetime::IsoDatetime;
use chrono::NaiveDateTime;

use chrono::ParseResult;
use crate::define_struct_vec_isodatetime;
define_struct_vec_isodatetime!(ArrayCycleAnchorDateOfPrincipalRedemption);
