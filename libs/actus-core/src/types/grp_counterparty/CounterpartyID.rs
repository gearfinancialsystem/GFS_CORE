use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type CounterpartyID = String;

// impl TermDescriptionTrait for CounterpartyID {
//     fn get_identifier(&self) -> &str {
//         "counterpartyID"
//     }
//     fn get_group(&self) -> &str {
//         "Counterparty"
//     }
//     fn get_name(&self) -> &str {
//         "Counterparty Identifier"
//     }
//     fn get_acronym(&self) -> &str {
//         "CPID"
//     }
//     fn get_type(&self) -> &str {
//         "Varchar"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "[]"
//     }
//     fn get_default_value(&self) -> &str {
//         ""
//     }
//     fn get_description(&self) -> &str {
//         "CPID identifies the counterparty to the CRID in this contract.
// CPID is ideally the official LEI which can be a firm, a government body, even a single person etc. However, this can also refer to a annonymous group in which case this information is not to be disclosed. CPID may also refer to a group taking a joint risk or more generally, CPID is the main counterparty, against which the contract has been settled."
//     }
// }    