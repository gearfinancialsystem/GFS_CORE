use std::str::FromStr;
use std::collections::HashMap;
use crate::util::Value::Value;
use crate::define_struct_f64;

define_struct_f64!(InterestScalingMultiplier, |value| {
}, {1.0});