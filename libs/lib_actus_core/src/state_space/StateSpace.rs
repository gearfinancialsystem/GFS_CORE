use crate::terms::grp_boundary::BoundaryCrossedFlag::BoundaryCrossedFlag;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_counterparty::contract_performance::Pf::PF;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::terms::grp_counterparty::NonPerformingDate::NonPerformingDate;
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_interest::AccruedInterest2::AccruedInterest2;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
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

pub type BoundaryMonitoringFlag = bool;
pub type BoundaryLeg1ActiveFlag = bool;
pub type BoundaryLeg2ActiveFlag = bool;

#[derive(Clone, PartialEq, Debug)]
pub struct StateSpace {
    pub accrued_interest: Option<AccruedInterest>,
    pub accrued_interest2: Option<AccruedInterest2>,
    pub contract_performance: Option<ContractPerformance>,
    pub exercise_amount: Option<ExerciseAmount>,
    pub exercise_date: Option<ExerciseDate>,
    pub fee_accrued: Option<FeeAccrued>,
    pub interest_calculation_base_amount: Option<InterestCalculationBaseAmount>,
    pub interest_scaling_multiplier: Option<InterestScalingMultiplier>,
    pub next_principal_redemption_payment: Option<NextPrincipalRedemptionPayment>,
    pub nominal_interest_rate: Option<NominalInterestRate>,
    pub nominal_interest_rate2: Option<NominalInterestRate2>,
    pub non_performing_date: Option<NonPerformingDate>,
    pub notional_principal: Option<NotionalPrincipal>,
    pub notional_principal2: Option<NotionalPrincipal2>,
    pub notional_scaling_multiplier: Option<NotionalScalingMultiplier>,
    pub status_date: Option<StatusDate>,
    pub maturity_date: Option<MaturityDate>,
    pub termination_date: Option<TerminationDate>,
    pub boundary_crossed_flag: Option<BoundaryCrossedFlag>,
    pub boundary_monitoring_flag: Option<BoundaryMonitoringFlag>, // pas dans la doc
    pub boundary_leg1_active_flag: Option<BoundaryLeg1ActiveFlag>,
    pub boundary_leg2_active_flag: Option<BoundaryLeg2ActiveFlag>,
    pub last_interest_period: Option<f64> // a voir
}

impl StateSpace {
    // Méthode pour créer une copie de StateSpace
    pub fn copy_state_space(original: &StateSpace) -> StateSpace {
        StateSpace {
            accrued_interest:       original.accrued_interest.clone(),
            accrued_interest2:      original.accrued_interest2.clone(),
            contract_performance:   original.contract_performance.clone(),
            exercise_amount:        original.exercise_amount.clone(),
            exercise_date:          original.exercise_date.clone(),
            fee_accrued:                        original.fee_accrued.clone(),
            interest_calculation_base_amount: original.interest_calculation_base_amount.clone(),
            interest_scaling_multiplier: original.interest_scaling_multiplier.clone(),
            next_principal_redemption_payment: original.next_principal_redemption_payment.clone(),
            nominal_interest_rate: original.nominal_interest_rate.clone(),
            nominal_interest_rate2: original.nominal_interest_rate2.clone(),
            non_performing_date: original.non_performing_date.clone(),
            notional_principal: original.notional_principal.clone(),
            notional_principal2: original.notional_principal2.clone(),
            notional_scaling_multiplier: original.notional_scaling_multiplier.clone(),
            status_date: original.status_date.clone(),
            maturity_date: original.maturity_date.clone(),
            termination_date: original.termination_date.clone(),
            boundary_crossed_flag: original.boundary_crossed_flag.clone(),
            boundary_monitoring_flag: original.boundary_monitoring_flag.clone(),
            boundary_leg1_active_flag: original.boundary_leg1_active_flag.clone(),
            boundary_leg2_active_flag: original.boundary_leg2_active_flag.clone(),
            last_interest_period: original.last_interest_period.clone(),
        }
    }
}
impl Default for StateSpace {
    fn default() -> Self {
        Self {
            accrued_interest: AccruedInterest::new(0.0).ok(),
            accrued_interest2: AccruedInterest2::new(0.0).ok(),
            contract_performance: ContractPerformance::new("PF").ok(),
            exercise_amount: ExerciseAmount::new(0.0).ok(),
            exercise_date: None,
            fee_accrued: FeeAccrued::new(0.0).ok(),
            interest_calculation_base_amount: InterestCalculationBaseAmount::new(0.0).ok(),
            interest_scaling_multiplier: InterestScalingMultiplier::new(0.0).ok(),
            next_principal_redemption_payment: NextPrincipalRedemptionPayment::new(0.0).ok(),
            nominal_interest_rate: NominalInterestRate::new(0.0).ok(),
            nominal_interest_rate2: NominalInterestRate2::new(0.0).ok(),
            non_performing_date: None,
            notional_principal: NotionalPrincipal::new(0.0).ok(),
            notional_principal2: NotionalPrincipal2::new(0.0).ok(),
            notional_scaling_multiplier: NotionalScalingMultiplier::new(0.0).ok(),
            status_date: None,
            maturity_date: None,
            termination_date: None,
            boundary_crossed_flag: BoundaryCrossedFlag::new(false).ok(),
            boundary_monitoring_flag: Some(false),
            boundary_leg1_active_flag: Some(false),
            boundary_leg2_active_flag: Some(false),
            last_interest_period: Some(0.0)
        }
    }
}