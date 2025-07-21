use crate::events::ContractEvent::ContractEvent;
use crate::events::EventType::EventType;
use crate::state_space::StateSpace::{BoundaryLeg1ActiveFlag, BoundaryLeg2ActiveFlag, BoundaryMonitoringFlag};
use crate::terms::grp_boundary::BoundaryCrossedFlag::BoundaryCrossedFlag;
use crate::terms::grp_contract_identification::ContractID::ContractID;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::terms::grp_counterparty::NonPerformingDate::NonPerformingDate;
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_interest::AccruedInterest2::AccruedInterest2;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
use crate::terms::grp_interest::NominalInterestRate2::NominalInterestRate2;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::terms::grp_notional_principal::Currency2::Currency2;
use crate::terms::grp_notional_principal::Currency::Currency;
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

#[derive(Debug, Clone, PartialEq)]
#[derive(Default)] // tout les champs a none
pub struct ResultSet {

    // comming from contract event
    pub epoch_offset: Option<i64>, // changer le inner type ?
    pub event_time: Option<IsoDatetime>,
    pub event_type: Option<EventType>,
    pub currency: Option<Currency>,
    pub contract_id: Option<ContractID>,

    // all below comming from states
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

impl ResultSet {
    pub fn new() -> Self {
        ResultSet::default()
    }

    pub fn set_result_set(&mut self, source_ce: &ContractEvent<IsoDatetime, IsoDatetime>) {
        let mut rs = ResultSet::new();

        // comming from contract event
        rs.epoch_offset = source_ce.epoch_offset;
        rs.event_time = source_ce.event_time;
        rs.event_type = Some(source_ce.event_type);
        rs.currency = source_ce.currency.clone();
        rs.contract_id = source_ce.contract_id.clone();

        // all below comming from states
        rs.accrued_interest = source_ce.state.accrued_interest.clone();
        rs.accrued_interest2 = source_ce.state.accrued_interest2.clone();
        rs.contract_performance = source_ce.state.contract_performance.clone();
        rs.exercise_amount = source_ce.state.exercise_amount.clone();
        rs.exercise_date = source_ce.state.exercise_date.clone();
        rs.fee_accrued = source_ce.state.fee_accrued.clone();
        rs.interest_calculation_base_amount = source_ce.state.interest_calculation_base_amount.clone();
        rs.interest_scaling_multiplier = source_ce.state.interest_scaling_multiplier.clone();
        rs.next_principal_redemption_payment = source_ce.state.next_principal_redemption_payment.clone();
        rs.nominal_interest_rate = source_ce.state.nominal_interest_rate.clone();
        rs.nominal_interest_rate2 = source_ce.state.nominal_interest_rate2.clone();
        rs.non_performing_date = source_ce.state.non_performing_date.clone();
        rs.notional_principal = source_ce.state.notional_principal.clone();
        rs.notional_principal2 = source_ce.state.notional_principal2.clone();
        rs.notional_scaling_multiplier = source_ce.state.notional_scaling_multiplier.clone();
        rs.status_date = source_ce.state.status_date.clone();
        rs.maturity_date = source_ce.state.maturity_date.clone();
        rs.termination_date = source_ce.state.termination_date.clone();
        rs.boundary_crossed_flag = source_ce.state.boundary_crossed_flag.clone();
        rs.boundary_monitoring_flag = source_ce.state.boundary_monitoring_flag.clone();
        rs.boundary_leg1_active_flag = source_ce.state.boundary_leg1_active_flag.clone();
        rs.boundary_leg2_active_flag = source_ce.state.boundary_leg2_active_flag.clone();
        rs.last_interest_period = source_ce.state.last_interest_period.clone();

    }
}
