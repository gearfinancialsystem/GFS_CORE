use std::ops::{Add, Sub};

use crate::types::IsoPeriod::IsoPeriod;
use crate::types::IsoDatetime::IsoDatetime;
use std::str::FromStr;
use std::collections::HashMap;
use crate::types::Value::Value;
use crate::define_struct_vec_isodatetime;
define_struct_vec_isodatetime!(ArrayCycleAnchorDateOfPrincipalRedemption);
