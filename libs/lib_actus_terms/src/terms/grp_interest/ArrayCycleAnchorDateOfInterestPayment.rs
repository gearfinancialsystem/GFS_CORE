
use std::ops::{Add, Sub};

use lib_actus_types::types::IsoPeriod::IsoPeriod;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use std::str::FromStr;
use std::collections::HashMap;
use lib_actus_types::types::Value::Value;

use crate::define_struct_vec_isodatetime;
define_struct_vec_isodatetime!(ArrayCycleAnchorDateOfInterestPayment);
