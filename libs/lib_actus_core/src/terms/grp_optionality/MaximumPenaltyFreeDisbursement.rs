use std::str::FromStr;
use crate::define_struct_f64;
define_struct_f64!(MaximumPenaltyFreeDisbursement, |value| {
    (value >= 0.0) => "value must be positive"
}, {}); // default should be the value of notional principal, a setup autre part