use std::collections::HashMap;
use std::str::FromStr;
use crate::util::ParseError::ParseError;

use crate::terms::grp_counterparty::contract_performance::Pf::PF;
use crate::terms::grp_counterparty::contract_performance::Dl::DL;
use crate::terms::grp_counterparty::contract_performance::Dq::DQ;
use crate::terms::grp_counterparty::contract_performance::Df::DF;
use crate::terms::grp_counterparty::contract_performance::Ma::MA;
use crate::terms::grp_counterparty::contract_performance::Te::TE;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractPerformance {
    PF(PF),
    DL(DL),
    DQ(DQ),
    DF(DF),
    MA(MA),
    TE(TE),
}

impl ContractPerformance {
    pub fn description(&self) -> String {
        match self {
            Self::PF(PF) => PF.type_str(),
            Self::DL(DL) => DL.type_str(),
            Self::DQ(DQ) => DQ.type_str(),
            Self::DF(DF) => DF.type_str(),
            Self::MA(MA) => MA.type_str(),
            Self::TE(TE) => TE.type_str()
        }
    }
    pub fn new_PF() -> Self {
        Self::PF(PF::new())
    }
    pub fn new_DL() -> Self {
        Self::DL(DL::new())
    }
    pub fn new_DQ() -> Self {
        Self::DQ(DQ::new())
    }
    pub fn new_DF() -> Self {
        Self::DF(DF::new())
    }
    pub fn new_MA() -> Self {
        Self::MA(MA::new())
    }
    pub fn new_TE() -> Self {
        Self::TE(TE::new())
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

impl FromStr for ContractPerformance {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            ""   => Ok(Self::default()),
            "PF" => Ok(Self::new_PF()),
            "DL" => Ok(Self::new_DL()),
            "DQ" => Ok(Self::new_DQ()),
            "DF" => Ok(Self::new_DF()),
            "MA" => Ok(Self::new_MA()),
            "TE" => Ok(Self::new_TE()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayConvention: {}", s)})
        }
    }
}

impl Default for ContractPerformance {
    fn default() -> Self {
        Self::new_PF()
    }
}

impl TermDescriptionTrait for ContractPerformance {
    fn get_identifier(&self) -> &str {
        "contractPerformance"
    }
    fn get_group(&self) -> &str {
        "Counterparty"
    }
    fn get_name(&self) -> &str {
        "Contract Performance"
    }
    fn get_acronym(&self) -> &str {
        "PRF"
    }
    fn get_type(&self) -> &str {
        "Enum"
    }
    fn get_allowed_values(&self) -> &str {
        "[{'option': '0', 'identifier': 'performant', 'name': 'Performant', 'acronym': 'PF', 'description': 'Contract is performing according to terms and conditions.\r'}, {'option': '1', 'identifier': 'delayed', 'name': 'Delayed', 'acronym': 'DL', 'description': 'Contractual payment obligations are delayed according to the Grace Period.\r'}, {'option': '2', 'identifier': 'delinquent', 'name': 'Delinquent', 'acronym': 'DQ', 'description': 'Contractual payment obligations are delinquent according to the Delinquency Period.\r'}, {'option': '3', 'identifier': 'default', 'name': 'Default', 'acronym': 'DF', 'description': 'Contract defaulted on payment obligations according to Delinquency Period.\r'}, {'option': '4', 'identifier': 'matured', 'name': 'Matured', 'acronym': 'MA', 'description': 'Contract matured.\r'}, {'option': '5', 'identifier': 'terminated', 'name': 'Terminated', 'acronym': 'TE', 'description': 'Contract has been terminated.'}]"
    }
    fn get_default_value(&self) -> &str {
        "PF"
    }
    fn get_description(&self) -> &str {
        "Indicates the current contract performance status. Different states of the contract range from performing to default."
    }
}