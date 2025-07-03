
use crate::types::IsoPeriod::IsoPeriod;
use chrono::NaiveDateTime;
use chrono::ParseResult;
use std::str::FromStr;
use std::collections::HashMap;
use crate::util::Value::Value;
use crate::types::IsoDatetime::IsoDatetime;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::events::AnyContractEvent::AnyContractEvent;
use crate::events::ContractEvent::ContractEvent;
use crate::traits::TraitConvertContractToAnyEvent::TraitConvertContractToAnyEvent;

use crate::define_struct_isodatetime;
define_struct_isodatetime!(StatusDate);