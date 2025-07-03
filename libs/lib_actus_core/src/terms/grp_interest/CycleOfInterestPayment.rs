

use crate::types::IsoCycle::IsoCycle;
use std::str::FromStr;
use crate::define_struct_isocycle;
use std::collections::HashMap;
use crate::util::Value::Value;
use crate::traits::TraitMarqueurIsoCycle::TraitMarqueurIsoCycle;
use crate::events::AnyContractEvent::AnyContractEvent;
use crate::events::ContractEvent::ContractEvent;
use crate::traits::TraitConvertContractToAnyEvent::TraitConvertContractToAnyEvent;
define_struct_isocycle!(CycleOfInterestPayment);

