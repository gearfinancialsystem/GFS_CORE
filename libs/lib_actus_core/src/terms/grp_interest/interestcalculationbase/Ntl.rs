use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(PartialEq, Eq, Debug)]
pub struct NTL;

impl NTL {
    pub fn new() -> Self {
        NTL
    }
}

impl TraitEnumOptionDescription for NTL {
    fn get_option_rank(&self) -> &str {
        "2"
    }
    fn get_identifier(&self) -> &str {
        "notionalLagged"
    }
    fn get_name(&self) -> &str {
        "Notional Outstanding Lagged"
    }
    fn get_acronym(&self) -> &str {
        "NTL"
    }
    fn get_description(&self) -> &str {
        "Interest accrues on the basis of the lagged notional outstanding."
    }
}  