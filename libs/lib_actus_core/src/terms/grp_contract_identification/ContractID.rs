use std::collections::HashMap;
use crate::traits::TraitTermDescription::TraitTermDescription;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)] // Derive traits as needed
pub struct ContractID(String);

impl ContractID {
    pub fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Box<Self> {
        string_map
            .get(key)
            .map(|s| Box::new(ContractID(s.clone())))
            .unwrap()
    }
}

impl TraitTermDescription for ContractID {
    fn get_identifier(&self) -> &str {
        "contractID"
    }
    fn get_group(&self) -> &str {
        "Contract identification"
    }
    fn get_name(&self) -> &str {
        "Contract Identifier"
    }
    fn get_acronym(&self) -> &str {
        "CID"
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
        "Unique identifier of a contract.  
If the system is used on a single firm level, an internal unique ID can be generated. If used on a national or globally level, a globally unique ID is required."
    }
}    