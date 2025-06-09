use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(PartialEq, Eq, Debug)]
pub struct NTIED;

impl NTIED {
    pub fn new() -> Self {
        NTIED
    }
}

impl TraitEnumOptionDescription for NTIED {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "notionalAtInitialExchange"
    }
    fn get_name(&self) -> &str {
        "Notional at Initial Exchange"
    }
    fn get_acronym(&self) -> &str {
        "NTIED"
    }
    fn get_description(&self) -> &str {
        "Interest accrues on the basis of the notional value at initial exchange."
    }
}