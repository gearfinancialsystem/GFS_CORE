

use crate::define_struct_f64;

// define_struct_f64!(NotionalPrincipal, |value| {
//     (value >= 0.0) => "value must be positive"
// }, {});

// jai desactivé la version précédente car NotionalPrincipal dans states space peut etre
// négatif (2eme leg dun swap par exemple)
define_struct_f64!(NotionalPrincipal, |value| {}, {});