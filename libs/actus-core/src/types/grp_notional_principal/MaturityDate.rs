use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type MaturityDate = IsoDatetime;

// impl TermDescriptionTrait for MaturityDate {
//     fn get_identifier(&self) -> &str {
//         "maturityDate"
//     }
//     fn get_group(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_name(&self) -> &str {
//         "Maturity Date"
//     }
//     fn get_acronym(&self) -> &str {
//         "MD"
//     }
//     fn get_type(&self) -> &str {
//         "Timestamp"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "['ISO8601 Datetime']"
//     }
//     fn get_default_value(&self) -> &str {
//         ""
//     }
//     fn get_description(&self) -> &str {
//         "Marks the contractual end of the lifecycle of a CT. Generally, date of the last cash flows. 
// This includes normally a principal and an interest payment. Some Maturity CTs as perpetuals (PBN) do not have such a date. For variable amortizing contracts of the ANN CT, this date might be less than the scheduled end of the contract (which is deduced from the periodic payment amount 
// PRNXT). In this case it balloons."
//     }
// }    