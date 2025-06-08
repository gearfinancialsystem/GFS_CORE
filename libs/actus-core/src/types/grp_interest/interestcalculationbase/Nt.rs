use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(PartialEq, Eq, Debug)]
pub struct NT;

impl NT {
    pub fn new() -> Self {
        NT
    }
}

impl TraitEnumOptionDescription for NT {
    fn get_option_rank(&self) -> &str {
        "0"
    }
    fn get_identifier(&self) -> &str {
        "notioalOutstanding"
    }
    fn get_name(&self) -> &str {
        "Notional Outstanding"
    }
    fn get_acronym(&self) -> &str {
        "NT"
    }
    fn get_description(&self) -> &str {
        "Interest accrues on the basis of the notional outstanding."
    }
}