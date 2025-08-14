use crate::phantom_terms::PhantomF64::PhantomF64W;
use crate::define_to_phantom_type_f64;
use crate::define_struct_f64;
define_struct_f64!(RateMultiplier, |value| {
}, {1.0});