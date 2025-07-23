use crate::terms::grp_notional_principal::Currency::Currency;
use std::str::FromStr;
use std::collections::HashMap;
use crate::types::Value::Value;

use crate::util::CurrencyValues::CURRENCIES;
use crate::define_struct_string;
define_struct_string!(Currency2, "currency");