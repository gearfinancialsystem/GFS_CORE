use std::str::FromStr;
use crate::define_struct_vec_f64;


define_struct_vec_f64!(ArrayNextPrincipalRedemptionPayment, |value| {
    (value >= 0.0 ) => "value must be positive"
}, {});

