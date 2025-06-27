use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use crate::exceptions::ParseError::ParseError;

use crate::terms::grp_notional_principal::scaling_effect::Ooo::OOO;
use crate::terms::grp_notional_principal::scaling_effect::Ono::ONO;
use crate::terms::grp_notional_principal::scaling_effect::Ioo::IOO;
use crate::terms::grp_notional_principal::scaling_effect::Ino::INO;

use crate::util::CommonUtils::CommonUtils as cu;
use crate::util::Value::Value;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ScalingEffect {
    OOO(OOO),
    IOO(IOO),
    ONO(ONO),
    INO(INO),
}

impl ScalingEffect {
    pub fn description(&self) -> String {
        match self {
            ScalingEffect::OOO(OOO) => OOO.type_str(),
            ScalingEffect::IOO(IOO) => IOO.type_str(),
            ScalingEffect::ONO(ONO) => ONO.type_str(),
            ScalingEffect::INO(INO) => INO.type_str(),
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

    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        cu::provide(string_map, key)
    }
}

impl fmt::Display for ScalingEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScalingEffect::OOO(_) => write!(f, "OOO"),
            ScalingEffect::IOO(_) => write!(f, "IOO"),
            ScalingEffect::ONO(_) => write!(f, "ONO"),
            ScalingEffect::INO(_) => write!(f, "INO"),
        }
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
            _ => Err(ParseError { message: format!("Invalid ScalingEffect: {}", s)})
        }
    }
}

impl Default for ScalingEffect {
    fn default() -> Self {
        ScalingEffect::OOO(OOO)
    }
}

