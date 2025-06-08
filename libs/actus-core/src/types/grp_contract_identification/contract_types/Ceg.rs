

#[derive(Debug, Eq, PartialEq)]
pub struct CEG;

impl CEG {
    pub fn new() -> Self {
        return CEG;
    }
    pub fn type_str(&self) -> String {
        return "CEG contract cont_type".to_string();
    }
}


impl TraitEnumOptionDescription for CEG {
    fn get_option_rank(&self) -> &str {
        "16"
    }
    fn get_identifier(&self) -> &str {
        "creditEnhancementGuarantee"
    }
    fn get_name(&self) -> &str {
        "Credit Enhancement Guarantee"
    }
    fn get_acronym(&self) -> &str {
        "CEG"
    }
    fn get_description(&self) -> &str {
        "A guarantee / letter of credit by a third party on the scheduled payment obligations of an underlying instrument"
    }
}    
        