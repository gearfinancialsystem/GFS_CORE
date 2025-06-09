use std::collections::HashMap;
use std::str::FromStr;
use crate::util::ParseError::ParseError;
use crate::terms::grp_contract_identification::contract_roles::Rpa::RPA;
use crate::terms::grp_contract_identification::contract_roles::Rpl::RPL;
use crate::terms::grp_contract_identification::contract_roles::Rfl::RFL;
use crate::terms::grp_contract_identification::contract_roles::Pfl::PFL;
use crate::terms::grp_contract_identification::contract_roles::Rf::RF;
use crate::terms::grp_contract_identification::contract_roles::Pf::PF;
use crate::terms::grp_contract_identification::contract_roles::Buy::BUY;
use crate::terms::grp_contract_identification::contract_roles::Sel::SEL;
use crate::terms::grp_contract_identification::contract_roles::Col::COL;
use crate::terms::grp_contract_identification::contract_roles::Cno::CNO;
use crate::terms::grp_contract_identification::contract_roles::Udl::UDL;
use crate::terms::grp_contract_identification::contract_roles::Udlp::UDLP;
use crate::terms::grp_contract_identification::contract_roles::Udlm::UDLM;
use crate::traits::TraitTermDescription::TraitTermDescription;

#[derive(Debug, PartialEq, Eq)]
pub enum ContractRole {
    RPA(RPA),
    RPL(RPL),
    RFL(RFL),
    PFL(PFL),
    RF(RF),
    PF(PF),
    BUY(BUY),
    SEL(SEL),
    COL(COL),
    CNO(CNO),
    UDL(UDL),
    UDLP(UDLP),
    UDLM(UDLM),
    None
}
impl ContractRole {
    pub fn description(&self) -> String {
        match self {
            Self::RPA(RPA) => RPA.type_str(),
            Self::RPL(RPL) => RPL.type_str(),
            Self::RFL(RFL) => RFL.type_str(),
            Self::PFL(PFL) => PFL.type_str(),
            Self::RF(RF) => RF.type_str(),
            Self::PF(PF) => PF.type_str(),
            Self::BUY(BUY) => BUY.type_str(),
            Self::SEL(SEL) => SEL.type_str(),
            Self::COL(COL) => COL.type_str(),
            Self::CNO(CNO) => CNO.type_str(),
            Self::UDL(UDL) => UDL.type_str(),
            Self::UDLP(UDLP) => UDLP.type_str(),
            Self::UDLM(UDLM) => UDLM.type_str(),
            Self::None => "None".to_string(),
        }
    }

    pub fn role_sign(&self) -> f64 {
        match self {
            Self::RPA(RPA) => RPA.role_sign(),
            Self::RPL(RPL) => RPL.role_sign(),
            Self::RFL(RFL) => RFL.role_sign(),
            Self::PFL(PFL) => PFL.role_sign(),
            Self::RF(RF) => RF.role_sign(),
            Self::PF(PF) => PF.role_sign(),
            Self::BUY(BUY) => BUY.role_sign(),
            Self::SEL(SEL) => SEL.role_sign(),
            Self::COL(COL) => COL.role_sign(),
            Self::CNO(CNO) => CNO.role_sign(),
            Self::UDL(UDL) => UDL.role_sign(),
            Self::UDLP(UDLP) => UDLP.role_sign(),
            Self::UDLM(UDLM) => UDLM.role_sign(),
            Self::None => 0.0,
        }
    }

    pub fn new_RPA() -> Self {
        Self::RPA(RPA::new())
    }
    pub fn new_RPL() -> Self {
        Self::RPL(RPL::new())
    }
    pub fn new_RFL() -> Self {
        Self::RFL(RFL::new())
    }
    pub fn new_PFL() -> Self {
        Self::PFL(PFL::new())
    }
    pub fn new_RF() -> Self {
        Self::RF(RF::new())
    }
    pub fn new_PF() -> Self {
        Self::PF(PF::new())
    }
    pub fn new_BUY() -> Self {
        Self::BUY(BUY::new())
    }
    pub fn new_SEL() -> Self {
        Self::SEL(SEL::new())
    }
    pub fn new_COL() -> Self {
        Self::COL(COL::new())
    }
    pub fn new_CNO() -> Self {
        Self::CNO(CNO::new())
    }
    pub fn new_UDL() -> Self {
        Self::UDL(UDL::new())
    }
    pub fn new_UDLP() -> Self {
        Self::UDLP(UDLP::new())
    }
    pub fn new_UDLM() -> Self {
        Self::UDLM(UDLM::new())
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

impl FromStr for ContractRole {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "RPA"  => Ok(Self::new_RPA()),
            "RPL"  => Ok(Self::new_RPL()),
            "RFL"  => Ok(Self::new_RFL()),
            "PFL"  => Ok(Self::new_PFL()),
            "RF"   => Ok(Self::new_RF()),
            "PF"   => Ok(Self::new_PF()),
            "BUY"  => Ok(Self::new_BUY()),
            "SEL"  => Ok(Self::new_SEL()),
            "COL"  => Ok(Self::new_COL()),
            "CNO"  => Ok(Self::new_CNO()),
            "UDL"  => Ok(Self::new_UDL()),
            "UDLP" => Ok(Self::new_UDLP()),
            "UDLM" => Ok(Self::new_UDLM()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayConvention: {}", s)})
        }
    }
}

impl Default for ContractRole {
    fn default() -> Self {
        Self::None
    }
}

impl TraitTermDescription for ContractRole {
    fn get_identifier(&self) -> &str {
        "contractRole"
    }
    fn get_group(&self) -> &str {
        "Contract identification"
    }
    fn get_name(&self) -> &str {
        "Contract Role"
    }
    fn get_acronym(&self) -> &str {
        "CNTRL"
    }
    fn get_type(&self) -> &str {
        "Enum"
    }
    fn get_allowed_values(&self) -> &str {
        "[{'option': '0', 'identifier': 'realPositionAsset', 'name': 'Real Position Asset', 'acronym': 'RPA', 'description': 'Contract creator takes the asset or lender side.\r'}, {'option': '1', 'identifier': 'realPositionLiability', 'name': 'Real Position Liability', 'acronym': 'RPL', 'description': 'Contract creator takes the liability or borrower side. \r'}, {'option': '2', 'identifier': 'receiveFirstLegl', 'name': 'Receive First Leg', 'acronym': 'RFL', 'description': 'Contract creator receives the first leg. \r'}, {'option': '3', 'identifier': 'payFirstLeg', 'name': 'Pay First Leg', 'acronym': 'PFL', 'description': 'Contract creator pays the first leg.\r'}, {'option': '4', 'identifier': 'receiveFix', 'name': 'Receive Fix', 'acronym': 'RF', 'description': 'Contract creator receives the fixed leg.\r'}, {'option': '5', 'identifier': 'payFix', 'name': 'Pay Fix', 'acronym': 'PF', 'description': 'Contract creator pays the fixed leg.\r'}, {'option': '6', 'identifier': 'buyer', 'name': 'Buyer', 'acronym': 'BUY', 'description': 'Contract creator holds the right to buy the underlying / exercise the option. \r'}, {'option': '7', 'identifier': 'seller', 'name': 'Seller', 'acronym': 'SEL', 'description': 'Contract creator holds the obligation to sell the underlying / deliver the option. \r'}, {'option': '8', 'identifier': 'collateralPosition', 'name': 'Collateral Position', 'acronym': 'COL', 'description': 'Contract represents a collateral to an underlying instrument\r'}, {'option': '9', 'identifier': 'closeOutNetting', 'name': 'Close out Netting', 'acronym': 'CNO', 'description': 'Contract creator and counterparty agree on netting payment obligations of underlying instruments in case of default. \r'}, {'option': '10', 'identifier': 'underlying', 'name': 'Underlying', 'acronym': 'UDL', 'description': 'Contract represents the underlying to a composed contract. Role of the underlying is derived from the parent. \r'}, {'option': '11', 'identifier': 'underlyingPlus', 'name': 'Underlying Plus', 'acronym': 'UDLP', 'description': 'Contract represents the underlying to a composed contract. Role of the underlying is derived from the parent. When considered a standalone contract the underlying’s creator takes the asset side. \r'}, {'option': '12', 'identifier': 'underlyingMinus', 'name': 'Underlying Minus', 'acronym': 'UDLM', 'description': 'Contract represents the underlying to a composed contract. Role of the underlying is derived from the parent. When considered a standalone contract the underlying’s creator takes the liability side.'}]"
    }
    fn get_default_value(&self) -> &str {
        ""
    }
    fn get_description(&self) -> &str {
        "CNTRL defines which position the CRID ( the creator of the contract record ) takes in a contract. For example, whether the contract is an asset or liability, a long or short position for the CRID. 
Most contracts are simple on or off balance sheet positions which are assets, liabilities. Such contracts can also play a secondary cont_role as a collateral. 
The attribute is highly significant since it determines the direction of all cash flows. The exact meaning is given with each CT in the ACTUS High Level Specification document."
    }
}    