use std::collections::HashMap;
use std::str::FromStr;
use crate::attributes::ContractReference::ContractReference;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;

#[derive(Clone, Debug, PartialEq)]
pub struct ContractStructure(Vec<ContractReference>);
// impl ContractStructure  {
//     pub fn provide(string_map: &HashMap<String, String>, key: &str) -> Option<Self> {
//         // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
//         string_map
//             .get(key)
//             .and_then(|s| {
//                 Self::from_str(s).ok()
//             })
//             .map(|b| b) // On stocke la convention dans une Box
//         //.unwrap_or_default()
//     }
// }
// 
// impl FromStr for ContractStructure {
//     type Err = ParseError;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s.to_uppercase().as_str() {
//             "SD" => Ok(Self::new_SD()),
//             "EOM" => Ok(Self::new_EOM()),
//             _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s)})
//         }
//     }
// }