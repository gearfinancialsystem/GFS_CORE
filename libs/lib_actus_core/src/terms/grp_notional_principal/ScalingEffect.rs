use std::collections::HashMap;
use std::str::FromStr;
use crate::util::ParseError::ParseError;

use crate::terms::grp_notional_principal::scaling_effect::Ooo::OOO;
use crate::terms::grp_notional_principal::scaling_effect::Ono::ONO;
use crate::terms::grp_notional_principal::scaling_effect::Ioo::IOO;
use crate::terms::grp_notional_principal::scaling_effect::Ino::INO;
use crate::traits::TraitTermDescription::TraitTermDescription;

#[derive(PartialEq, Eq, Debug)]
pub enum ScalingEffect {
    OOO(OOO),
    IOO(IOO),
    ONO(ONO),
    INO(INO),
    None
}

impl ScalingEffect {
    pub fn description(&self) -> String {
        match self {
            ScalingEffect::OOO(OOO) => OOO.type_str(),
            ScalingEffect::IOO(IOO) => IOO.type_str(),
            ScalingEffect::ONO(ONO) => ONO.type_str(),
            ScalingEffect::INO(INO) => INO.type_str(),
            ScalingEffect::None => "None".to_string(),
        }
    }
    pub fn new_OOO() -> Self {
        ScalingEffect::OOO(OOO::new())
    }
    pub fn new_RPL() -> Self {
        ScalingEffect::IOO(IOO::new())
    }
    pub fn new_RFL() -> Self {
        ScalingEffect::ONO(ONO::new())
    }
    pub fn new_PFL() -> Self {
        ScalingEffect::INO(INO::new())
    }

    pub fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Box<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                ScalingEffect::from_str(s).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box
            .unwrap_or_default()
    }
}

impl FromStr for ScalingEffect {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "OOO" => Ok(ScalingEffect::OOO(OOO)),
            "IOO" => Ok(ScalingEffect::IOO(IOO)),
            "ONO" => Ok(ScalingEffect::ONO(ONO)),
            "INO" => Ok(ScalingEffect::INO(INO)),
            _ => Err(ParseError { message: format!("Invalid BusinessDayConvention: {}", s)})
        }
    }
}

impl Default for ScalingEffect {
    fn default() -> Self {
        ScalingEffect::None
    }
}

impl TraitTermDescription for ScalingEffect {
    fn get_identifier(&self) -> &str {
        "scalingEffect"
    }
    fn get_group(&self) -> &str {
        "Notional Principal"
    }
    fn get_name(&self) -> &str {
        "Scaling Effect"
    }
    fn get_acronym(&self) -> &str {
        "SCEF"
    }
    fn get_type(&self) -> &str {
        "Enum"
    }
    fn get_allowed_values(&self) -> &str {
        "[{'option': '0', 'identifier': 'noScaling', 'name': 'No Scaling', 'acronym': '000', 'description': 'No scaling applies.\r'}, {'option': '1', 'identifier': 'interestIsScaled', 'name': 'Interest is Scaled', 'acronym': 'I00', 'description': 'Scaling applies only to interest.\r'}, {'option': '2', 'identifier': 'principalIsScaled', 'name': 'Principal is Scaled', 'acronym': '0N0', 'description': 'Scaling applies only to principal.\r'}, {'option': '3', 'identifier': 'interestAndPrincipalIsScaled', 'name': 'Interest and Principal is Scaled', 'acronym': 'IN0', 'description': 'Scaling applies to interest and principal.\r'}]"
    }
    fn get_default_value(&self) -> &str {
        "000
"
    }
    fn get_description(&self) -> &str {
        "Indicates which payments are scaled. I = Interest payments, N = Nominal payments and M = Maximum deferred interest amount. They can be scaled in any combination."
    }
}    