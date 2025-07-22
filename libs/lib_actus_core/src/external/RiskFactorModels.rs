use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::str::FromStr;
use crate::attributes::ContractTerms::ContractTerms;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventSequence::EventSequence;
use crate::state_space::StateSpace::StateSpace;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::types::IsoDatetime::IsoDatetime;
use crate::events::EventType::EventType;
use crate::external::data_observers::DataObserver1::DataObserver1;
use crate::terms::grp_contract_identification::ContractID::ContractID;
use crate::terms::grp_notional_principal::Currency::Currency;
use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;

use crate::external::EventObserved::EventObserved;
use crate::external::DataObserved::DataObserved;

#[derive(Debug, Clone, PartialEq)]
pub enum RiskFactorModels {
    RiskFactorModel1(RiskFactorModel1)
}

