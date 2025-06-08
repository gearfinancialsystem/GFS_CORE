use crate::subtypes::RealPositive::RealPositive;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type Quantity = RealPositive;

// impl TermDescriptionTrait for Quantity {
//     fn get_identifier(&self) -> &str {
//         "quantity"
//     }
//     fn get_group(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_name(&self) -> &str {
//         "Quantity"
//     }
//     fn get_acronym(&self) -> &str {
//         "QT"
//     }
//     fn get_type(&self) -> &str {
//         "Real"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "['Positive']"
//     }
//     fn get_default_value(&self) -> &str {
//         "1"
//     }
//     fn get_description(&self) -> &str {
//         "This attribute relates either to physical contracts (COM) or underlyings of traded contracts. 
// In case of physical contracts it holds the number of underlying units of the specific good (e.g. number of barrels of oil). 
// In case of well defined traded contracts it holds the number of defined underlying instruments. Example: QT of STK CTs underlying a FUTUR indicates the number of those specific STK CTs which underlie the FUTUR."
//     }
// }   