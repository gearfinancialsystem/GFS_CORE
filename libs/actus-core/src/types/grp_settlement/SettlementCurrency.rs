use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type SettlementCurrency = String;

// impl TermDescriptionTrait for SettlementCurrency {
//     fn get_identifier(&self) -> &str {
//         "settlementCurrency"
//     }
//     fn get_group(&self) -> &str {
//         "Settlement"
//     }
//     fn get_name(&self) -> &str {
//         "Settlement Currency"
//     }
//     fn get_acronym(&self) -> &str {
//         "CURS"
//     }
//     fn get_type(&self) -> &str {
//         "Varchar"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "['ISO4217']"
//     }
//     fn get_default_value(&self) -> &str {
//         ""
//     }
//     fn get_description(&self) -> &str {
//         "The currency in which cash flows are settled. This currency can be different from the currency (CUR) in which cash flows or the contract, respectively, is denominated in which case the respective FX-rate applies at settlement time.
// If no settlement currency is defined the cash flows are settled in the currency in which they are denominated."
//     }
// }    