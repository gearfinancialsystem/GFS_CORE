
use std::str::FromStr;
use std::collections::HashMap;
use crate::util::Value::Value;
use crate::types::IsoPeriod::IsoPeriod;

use crate::define_struct_isoperiod;

define_struct_isoperiod!(FixingPeriod);
