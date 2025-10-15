use crate::phantom_terms::PhantomF64::PhantomF64W;
//  use crate::define_to_phantom_type_f64;
use crate::define_struct_f64;

define_struct_f64!(MaintenanceMarginLowerBound, |value| {
    (value >= 0.0) => "value must be positive"
}, {});