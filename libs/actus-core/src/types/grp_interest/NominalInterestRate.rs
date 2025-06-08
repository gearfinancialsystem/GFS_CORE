use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type NominalInterestRate = f64;

// impl TermDescriptionTrait for NominalInterestRate {
//     fn get_identifier(&self) -> &str {
//         "nominalInterestRate"
//     }
//     fn get_group(&self) -> &str {
//         "Interest"
//     }
//     fn get_name(&self) -> &str {
//         "Nominal Interest Rate"
//     }
//     fn get_acronym(&self) -> &str {
//         "IPNR"
//     }
//     fn get_type(&self) -> &str {
//         "Real"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "[]"
//     }
//     fn get_default_value(&self) -> &str {
//         ""
//     }
//     fn get_description(&self) -> &str {
//         "The nominal interest rate which will be used to calculate accruals and the next interest payment at the next IP date. NT multiplied with IPNR is the base for the interest payment calculation. The relevant time period is a function of IPDC. 
// If the contract is variable (RRANX set) this field is periodically updated per SD. 
// In the case of plan vanilla interest rate swaps (IRSPV) this defines the rate of fixed leg."
//     }
// }    