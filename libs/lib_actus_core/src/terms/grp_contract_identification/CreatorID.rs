use crate::traits::TraitTermDescription::TraitTermDescription;

pub type CreatorID = String;

impl TraitTermDescription for CreatorID {
    fn get_identifier(&self) -> &str {
        "creatorID"
    }
    fn get_group(&self) -> &str {
        "Contract identification"
    }
    fn get_name(&self) -> &str {
        "Creator Identifier"
    }
    fn get_acronym(&self) -> &str {
        "CRID"
    }
    fn get_type(&self) -> &str {
        "Varchar"
    }
    fn get_allowed_values(&self) -> &str {
        "[]"
    }
    fn get_default_value(&self) -> &str {
        ""
    }
    fn get_description(&self) -> &str {
        "This identifies the legal entity creating the contract record. The counterparty of the contract is tracked in CPID.
CRID is ideally the official LEI which can be a firm, a government body, even a single person etc. However, this can also refer to a annonymous group in which case this information is not to be disclosed. CRID may also refer to a group taking a joint risk."
    }
}   