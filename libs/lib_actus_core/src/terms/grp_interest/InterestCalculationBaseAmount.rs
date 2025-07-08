
use crate::define_struct_f64;

define_struct_f64!(InterestCalculationBaseAmount, |value| {
    (value >= 0.0) => "value must be positive"
}, {});