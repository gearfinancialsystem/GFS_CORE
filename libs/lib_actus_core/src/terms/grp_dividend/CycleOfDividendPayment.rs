use std::str::FromStr;
use std::collections::HashMap;
use crate::util::Value::Value;
use crate::types::IsoCycle::IsoCycle;
use crate::define_struct_isocycle;
use crate::traits::TraitMarqueurIsoCycle::TraitMarqueurIsoCycle;
define_struct_isocycle!(CycleOfDividendPayment);