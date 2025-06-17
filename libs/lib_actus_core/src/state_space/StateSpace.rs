use std::fmt;
use crate::terms::grp_counterparty::contract_performance::Pf::PF;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::types::isoDatetime::IsoDatetime;

pub type BoundaryLeg1ActiveFlag = bool;
pub type BoundaryLeg2ActiveFlag = bool;

#[derive(Clone, PartialEq, Debug)]
pub struct StateSpace {
    pub accruedInterest: Option<f64>,
    pub accruedInterest2: Option<f64>,
    pub contractPerformance: Option<ContractPerformance>,
    pub exerciseAmount: Option<f64>,
    pub exerciseDate: Option<IsoDatetime>,
    pub feeAccrued: Option<f64>,
    pub interestCalculationBaseAmount: Option<f64>,
    pub interestScalingMultiplier: Option<f64>,
    pub nextPrincipalRedemptionPayment: Option<f64>,
    pub nominalInterestRate: Option<f64>,
    pub nominalInterestRate2: Option<f64>,
    pub nonPerformingDate: Option<IsoDatetime>,
    pub notionalPrincipal: Option<f64>,
    pub notionalPrincipal2: Option<f64>,
    pub notionalScalingMultiplier: Option<f64>,
    pub statusDate: Option<IsoDatetime>,
    pub maturityDate: Option<IsoDatetime>,
    pub terminationDate: Option<IsoDatetime>,
    pub boundaryCrossedFlag: Option<bool>,
    pub boundaryMonitoringFlag: Option<bool>,
    pub boundaryLeg1ActiveFlag: Option<bool>,
    pub boundaryLeg2ActiveFlag: Option<bool>,
    pub lastInterestPeriod: Option<f64>
}

impl StateSpace {
    // Méthode pour créer une copie de StateSpace
    pub fn copy_state_space(original: &StateSpace) -> StateSpace {
        StateSpace {
            accruedInterest: original.accruedInterest,
            accruedInterest2: original.accruedInterest2,
            contractPerformance: original.contractPerformance.clone(),
            exerciseAmount: original.exerciseAmount,
            exerciseDate: original.exerciseDate,
            feeAccrued: original.feeAccrued,
            interestCalculationBaseAmount: original.interestCalculationBaseAmount,
            interestScalingMultiplier: original.interestScalingMultiplier,
            nextPrincipalRedemptionPayment: original.nextPrincipalRedemptionPayment,
            nominalInterestRate: original.nominalInterestRate,
            nominalInterestRate2: original.nominalInterestRate2,
            nonPerformingDate: original.nonPerformingDate,
            notionalPrincipal: original.notionalPrincipal,
            notionalPrincipal2: original.notionalPrincipal2,
            notionalScalingMultiplier: original.notionalScalingMultiplier,
            statusDate: original.statusDate,
            maturityDate: original.maturityDate,
            terminationDate: original.terminationDate,
            boundaryCrossedFlag: original.boundaryCrossedFlag,
            boundaryMonitoringFlag: original.boundaryMonitoringFlag,
            boundaryLeg1ActiveFlag: original.boundaryLeg1ActiveFlag,
            boundaryLeg2ActiveFlag: original.boundaryLeg2ActiveFlag,
            lastInterestPeriod: original.lastInterestPeriod,
        }
    }
}
impl Default for StateSpace {
    fn default() -> Self {
        Self {
            accruedInterest: Some(0.0),
            accruedInterest2: Some(0.0),
            contractPerformance: Some(ContractPerformance::PF(PF::new())),
            exerciseAmount: Some(0.0),
            exerciseDate: None,
            feeAccrued: Some(0.0),
            interestCalculationBaseAmount: Some(0.0),
            interestScalingMultiplier: Some(0.0),
            nextPrincipalRedemptionPayment: Some(0.0),
            nominalInterestRate: Some(0.0),
            nominalInterestRate2: Some(0.0),
            nonPerformingDate: None,
            notionalPrincipal: Some(0.0),
            notionalPrincipal2: Some(0.0),
            notionalScalingMultiplier: Some(0.0),
            statusDate: None,
            maturityDate: None,
            terminationDate: None,
            boundaryCrossedFlag: Some(false),
            boundaryMonitoringFlag: Some(false),
            boundaryLeg1ActiveFlag: Some(false),
            boundaryLeg2ActiveFlag: Some(false),
            lastInterestPeriod: Some(0.0)
        }
    }
}