
use crate::types::IsoDatetime::IsoDatetime;
use crate::types::IsoPeriod::IsoPeriod;
use chrono::NaiveDateTime;
use chrono::ParseResult;
use std::collections::HashMap;
use std::str::FromStr;
use crate::util::Value::Value;
use crate::define_struct_isodatetime;
define_struct_isodatetime!(CapitalizationEndDate);