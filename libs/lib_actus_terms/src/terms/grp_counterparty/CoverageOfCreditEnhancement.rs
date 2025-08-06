
use crate::define_to_phantom_type_f64;
use crate::define_phantom_imports_f64;
use crate::define_struct_f64;


define_struct_f64!(CoverageOfCreditEnhancement, |value| {
    (value >= 0.0 && value <= 1.0) => "value must be between 0 and 1.0.",
},
{1.0});