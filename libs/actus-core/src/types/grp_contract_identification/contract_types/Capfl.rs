

#[derive(Debug, Eq, PartialEq)]
pub struct CAPFL;

impl CAPFL {
    pub fn new() -> Self {
        return CAPFL;
    }
    pub fn type_str(&self) -> String {
        return "CAPFL contract cont_type".to_string();
    }
}


impl TraitEnumOptionDescription for CAPFL {
    fn get_option_rank(&self) -> &str {
        "13"
    }
    fn get_identifier(&self) -> &str {
        "capFloor"
    }
    fn get_name(&self) -> &str {
        "Cap and Floor"
    }
    fn get_acronym(&self) -> &str {
        "CAPFL"
    }
    fn get_description(&self) -> &str {
        "An agreement of paying the differential (cap or floor) of a reference rate versus a fixed rate."
    }
}   