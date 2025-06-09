use crate::traits::TraitTermDescription::TraitTermDescription;

#[derive(Debug, PartialEq)]
pub struct ArrayRate {
    list_value: Option<Vec<f64>>
}

impl Default for ArrayRate {
    fn default() -> Self {
        Self {
            list_value: None,
        }
    }
}

impl TraitTermDescription for ArrayRate {
    fn get_identifier(&self) -> &str {
        "arrayRate"
    }
    fn get_group(&self) -> &str {
        "Rate Reset"
    }
    fn get_name(&self) -> &str {
        "Array Rate"
    }
    fn get_acronym(&self) -> &str {
        "ARRATE"
    }
    fn get_type(&self) -> &str {
        "Real[]"
    }
    fn get_allowed_values(&self) -> &str {
        "[]"
    }
    fn get_default_value(&self) -> &str {
        ""
    }
    fn get_description(&self) -> &str {
        "For array-cont_type rate reset schedules, this attribute represents either an interest rate (corresponding to IPNR) or a spread (corresponding to RRSP). Which case applies depends on the attribute ARFIXVAR: if ARFIXVAR=FIX then it represents the new IPNR and if ARFIXVAR=VAR then the applicable RRSP."
    }
}  
