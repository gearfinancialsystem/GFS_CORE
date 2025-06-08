use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_notional_principal::ArrayIncreaseDecrease::ArrayIncreaseDecrease;
use crate::terms::grp_notional_principal::increase_decrease::DEC::DEC;
use crate::terms::grp_notional_principal::increase_decrease::INC::INC;
use crate::terms::grp_notional_principal::unit::BRL::BRL;
use crate::terms::grp_notional_principal::unit::BSH::BSH;
use crate::terms::grp_notional_principal::unit::CUU::CUU;
use crate::terms::grp_notional_principal::unit::GLN::GLN;
use crate::terms::grp_notional_principal::unit::MWH::MWH;
use crate::terms::grp_notional_principal::unit::PND::PND;
use crate::terms::grp_notional_principal::unit::STN::STN;
use crate::terms::grp_notional_principal::unit::TON::TON;
use crate::terms::grp_notional_principal::unit::TRO::TRO;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;
use crate::util::ParseError::ParseError;

pub enum Unit {
    BRL(BRL),
    BSH(BSH),
    GLN(GLN),
    CUU(CUU),
    MWH(MWH),
    PND(PND),
    STN(STN),
    TON(TON),
    TRO(TRO),
    None
}

impl Unit {
    pub fn description(&self) -> String {
        match self {
            Self::BRL(BRL) => BRL.type_str(),
            Self::BSH(BSH) => BSH.type_str(),
            Self::GLN(GLN) => GLN.type_str(),
            Self::CUU(CUU) => CUU.type_str(),
            Self::MWH(MWH) => MWH.type_str(),
            Self::PND(PND) => PND.type_str(),
            Self::STN(STN) => STN.type_str(),
            Self::TON(TON) => TON.type_str(),
            Self::TRO(TRO) => TRO.type_str(),
            Self::None => "".to_string(),
        }
    }
    pub fn new_BRL() -> Self {
        Self::BRL(BRL::new())
    }
    pub fn new_BSH() -> Self {
        Self::BSH(BSH::new())
    }
    pub fn new_GLN() -> Self {
        Self::GLN(GLN::new())
    }
    pub fn new_CUU() -> Self {
        Self::CUU(CUU::new())
    }
    pub fn new_MWH() -> Self {
        Self::MWH(MWH::new())
    }
    pub fn new_PND() -> Self {
        Self::PND(PND::new())
    }
    pub fn new_STN() -> Self {
        Self::STN(STN::new())
    }
    pub fn new_TON() -> Self {
        Self::TON(TON::new())
    }
    pub fn new_TRO() -> Self {
        Self::TRO(TRO::new())
    }
    pub fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Box<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                Self::from_str(s).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box
            .unwrap_or_default()
    }
}

impl Default for Unit {
    fn default() -> Self {
        Self::None
    }
}

impl FromStr for Unit {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "BRL" => Ok(Unit::new_BRL()),
            "BSH" => Ok(Unit::new_BSH()),
            "GLN" => Ok(Unit::new_GLN()),
            "CUU" => Ok(Unit::new_CUU()),
            "MWH" => Ok(Unit::new_MWH()),
            "PND" => Ok(Unit::new_PND()),
            "STN" => Ok(Unit::new_STN()),
            "TON" => Ok(Unit::new_TON()),
            "TRO" => Ok(Unit::new_TRO()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayConvention: {}", s)})
        }
    }
}




impl TermDescriptionTrait for Unit {
    fn get_identifier(&self) -> &str {
        "unit"
    }
    fn get_group(&self) -> &str {
        "Notional Principal"
    }
    fn get_name(&self) -> &str {
        "Unit"
    }
    fn get_acronym(&self) -> &str {
        "UT"
    }
    fn get_type(&self) -> &str {
        "Enum"
    }
    fn get_allowed_values(&self) -> &str {
        "[{'option': '0', 'identifier': 'barrel', 'name': 'Barrel', 'acronym': 'BRL', 'description': 'Physical unit of the contract is Barrels.\r'}, {'option': '1', 'identifier': 'bushel', 'name': 'Bushel', 'acronym': 'BSH', 'description': 'Physical unit of the contract is Bushel.\r'}, {'option': '2', 'identifier': 'gallon', 'name': 'Gallon', 'acronym': 'GLN', 'description': 'Physical unit of the contract is Gallons.\r'}, {'option': '3', 'identifier': 'currencyUnit', 'name': 'Currency Unit', 'acronym': 'CUU', 'description': 'Physical unit of the contract is Currency Units.\r'}, {'option': '4', 'identifier': 'megaWattHours', 'name': 'Mega Watt Hours', 'acronym': 'MWH', 'description': 'Physical unit of the contract is Mega Watt Hours.\r'}, {'option': '5', 'identifier': 'pounds', 'name': 'Pounds', 'acronym': 'PND', 'description': 'Physical unit of the contract is Pounds.\r'}, {'option': '6', 'identifier': 'shortTons', 'name': 'Short Tons', 'acronym': 'STN', 'description': 'Physical unit of the contract is Short Tons.\r'}, {'option': '7', 'identifier': 'tons', 'name': 'Tons', 'acronym': 'TON', 'description': 'Physical unit of the contract is Tons.\r'}, {'option': '8', 'identifier': 'troyOunce', 'name': 'Troy Ounce', 'acronym': 'TRO', 'description': 'Physical unit of the contract is Troy Ounces.'}]"
    }
    fn get_default_value(&self) -> &str {
        ""
    }
    fn get_description(&self) -> &str {
        "The physical unit of the contract. Example: Barrels for an Oil COM CT."
    }
}