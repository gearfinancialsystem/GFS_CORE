
use crate::types::IsoDatetime::IsoDatetime;
use crate::types::IsoPeriod::IsoPeriod;
use chrono::NaiveDateTime;
use chrono::ParseResult;
use std::collections::HashMap;
use std::str::FromStr;
use crate::util::Value::Value;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::define_struct_isodatetime;
use crate::events::AnyContractEvent::AnyContractEvent;
use crate::events::ContractEvent::ContractEvent;
use crate::traits::TraitConvertContractToAnyEvent::TraitConvertContractToAnyEvent;
define_struct_isodatetime!(CapitalizationEndDate);