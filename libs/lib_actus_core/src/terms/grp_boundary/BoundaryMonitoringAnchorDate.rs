use crate::types::IsoDatetime::IsoDatetime;
use crate::types::IsoPeriod::IsoPeriod;
use chrono::NaiveDateTime;
use chrono::ParseResult;
use std::str::FromStr;
use std::collections::HashMap;
use crate::util::Value::Value;
use crate::define_struct_isodatetime;
define_struct_isodatetime!(BoundaryMonitoringAnchorDate);

