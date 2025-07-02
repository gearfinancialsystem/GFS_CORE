
use std::str::FromStr;
use std::collections::HashMap;
use crate::util::Value::Value;
use crate::define_struct_vec_f64;

define_struct_vec_f64!(ArrayRate, |value| {
    (value >= 0.0 ) => "value must be positive"
}, {});

