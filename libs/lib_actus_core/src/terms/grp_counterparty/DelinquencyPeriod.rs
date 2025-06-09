use crate::subtypes::IsoDuration::IsoDuration;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type DelinquencyPeriod = IsoDuration;

impl TraitTermDescription for DelinquencyPeriod {
    fn get_identifier(&self) -> &str {
        "delinquencyPeriod"
    }
    fn get_group(&self) -> &str {
        "Counterparty"
    }
    fn get_name(&self) -> &str {
        "Delinquency Period"
    }
    fn get_acronym(&self) -> &str {
        "DQP"
    }
    fn get_type(&self) -> &str {
        "Period"
    }
    fn get_allowed_values(&self) -> &str {
        "['ISO8601 Duration']"
    }
    fn get_default_value(&self) -> &str {
        "P0D"
    }
    fn get_description(&self) -> &str {
        "If real payment happens after scheduled payment date plus DLP, then the counterparty is in technical default. This means that the creditor legally has the right to declare default of the debtor."
    }
}    