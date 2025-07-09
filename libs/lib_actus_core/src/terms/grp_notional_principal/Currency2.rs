use crate::terms::grp_notional_principal::Currency::Currency;
use std::str::FromStr;
use std::collections::HashMap;
use crate::util::Value::Value;
use crate::util::CommonUtils::CURRENCIES;
use crate::define_struct_string;
define_struct_string!(Currency2, "currency");