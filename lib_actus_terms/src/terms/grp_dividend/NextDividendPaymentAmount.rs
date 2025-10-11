use crate::define_to_phantom_type_f64;
use crate::define_phantom_imports_f64;
use crate::define_struct_f64;

define_struct_f64!(NextDividendPaymentAmount, |value| {
    (value >= 0.0) => "value must be positive"
}, {0.0});
