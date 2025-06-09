use crate::terms::grp_reset_rate::cycle_point_of_rate_reset::B::B;
use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct E;

impl E {
    pub fn new() -> Self {
        return E;
    }
    pub fn type_str(&self) -> String {
        return "E Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for E {
    /// Return elements as defined in https://github.com/actusfrf/actus-dictionary/blob/master/actus-dictionary-terms.json
    fn get_option_rank(&self) -> &str {
        return "1";
    }
    fn get_identifier(&self) -> &str {
        return "end";
    }
    fn get_name(&self) -> &str {
        return "End";
    }
    fn get_acronym(&self) -> &str {
        return "E";
    }
    fn get_description(&self) -> &str {
        "The new rate is applied at the end of the reset period."
    }
}