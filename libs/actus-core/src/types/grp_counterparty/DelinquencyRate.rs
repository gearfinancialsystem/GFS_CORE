use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub struct DelinquencyRate(f64);

impl TermDescriptionTrait for DelinquencyRate {
    fn get_identifier(&self) -> &str {
        "delinquencyRate"
    }
    fn get_group(&self) -> &str {
        "Counterparty"
    }
    fn get_name(&self) -> &str {
        "Delinquency Rate"
    }
    fn get_acronym(&self) -> &str {
        "DQR"
    }
    fn get_type(&self) -> &str {
        "Real"
    }
    fn get_allowed_values(&self) -> &str {
        "['Positive']"
    }
    fn get_default_value(&self) -> &str {
        "0"
    }
    fn get_description(&self) -> &str {
        "Rate at which Delinquency Payments accrue on NT (in addition to the interest rate) during the DelinquencyPeriod"
    }
}    