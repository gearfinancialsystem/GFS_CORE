use serde_json::{self, Value as JsonValue};
use std::fs::File;
use std::io::BufReader;

use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_boundary::BoundaryCrossedFlag::BoundaryCrossedFlag;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::terms::grp_counterparty::NonPerformingDate::NonPerformingDate;
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_interest::AccruedInterest2::AccruedInterest2;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
use crate::terms::grp_interest::NominalInterestRate2::NominalInterestRate2;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::terms::grp_notional_principal::InterestScalingMultiplier::InterestScalingMultiplier;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::NextPrincipalRedemptionPayment::NextPrincipalRedemptionPayment;
use crate::terms::grp_notional_principal::NotionalPrincipal2::NotionalPrincipal2;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;
use crate::terms::grp_notional_principal::TerminationDate::TerminationDate;
use crate::terms::grp_settlement::ExerciseAmount::ExerciseAmount;
use crate::terms::grp_settlement::ExerciseDate::ExerciseDate;
use crate::types::IsoDatetime::IsoDatetime;
use std::str::FromStr;
#[derive(PartialEq, Debug, Clone)]
pub struct EventObserver1 {
    time: String,
    typex: String,
    value: f64,
    contract_id: String,
    states: StateSpace,
}
impl EventObserver1 {
    pub fn new(time: String,
               typex: String,
               value: f64,
               contract_id: String,
               states: StateSpace) -> Self {
        Self {time, typex, value, contract_id, states}
    }
    
    pub fn get_contract_id(&self) -> String {
        self.contract_id.clone()
    }
    
    pub fn set_contract_id(&mut self, contract_id: String) {
        self.contract_id = contract_id;
    }
    
    pub fn get_states(&self) -> StateSpace {
        self.states.clone()
    }
    
    pub fn set_states(&mut self, states: StateSpace) {
        self.states = states;
    }
    
    pub fn get_time(&self) -> String {
        self.time.clone()
    }
    pub fn set_time(&mut self, time: String) {
        self.time = time;
    }
    
    pub fn get_typex(&self) -> String {
        self.typex.clone()
    }
    
    pub fn set_typex(&mut self, typex: String) {
        self.typex = typex;
    }
    
    pub fn get_value(&self) -> f64 {
        self.value
    }
}




