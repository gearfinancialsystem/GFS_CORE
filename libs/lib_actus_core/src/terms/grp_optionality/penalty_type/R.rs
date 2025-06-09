use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct R;

impl R {
    pub fn new() -> Self {
        return R;
    }
    pub fn type_str(&self) -> String {
        return "R Scaling Effect".to_string();
    }
}


impl TraitEnumOptionDescription for R {
    fn get_option_rank(&self) -> &str {
        "2"
    }
    fn get_identifier(&self) -> &str {
        "relativePenalty"
    }
    fn get_name(&self) -> &str {
        "Relative Penalty"
    }
    fn get_acronym(&self) -> &str {
        "R"
    }
    fn get_description(&self) -> &str {
        "A penalty relative to the notional outstanding applies."
    }
}