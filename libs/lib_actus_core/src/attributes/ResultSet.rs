use crate::events::ContractEvent::ContractEvent;
use crate::events::EventType::EventType;
use crate::state_space::StateSpace::{BoundaryLeg1ActiveFlag, BoundaryLeg2ActiveFlag, BoundaryMonitoringFlag, StateSpace};
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
    pub payoff: Option<f64>,

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

    pub fn set_result_set(&mut self, curr_states: &StateSpace,  source_ce: &ContractEvent<IsoDatetime, IsoDatetime>) {
        
        // comming from contract event
        self.epoch_offset = source_ce.epoch_offset;
        self.event_time = source_ce.event_time;
        self.event_type = Some(source_ce.event_type);
        self.currency = source_ce.currency.clone();
        self.contract_id = source_ce.contract_id.clone();
        self.payoff = source_ce.payoff.clone();

        // all below comming from states
        self.accrued_interest = curr_states.accrued_interest.clone();
        self.accrued_interest2 = curr_states.accrued_interest2.clone();
        self.contract_performance = curr_states.contract_performance.clone();
        self.exercise_amount = curr_states.exercise_amount.clone();
        self.exercise_date = curr_states.exercise_date.clone();
        self.fee_accrued = curr_states.fee_accrued.clone();
        self.interest_calculation_base_amount = curr_states.interest_calculation_base_amount.clone();
        self.interest_scaling_multiplier = curr_states.interest_scaling_multiplier.clone();
        self.next_principal_redemption_payment = curr_states.next_principal_redemption_payment.clone();
        self.nominal_interest_rate = curr_states.nominal_interest_rate.clone();
        self.nominal_interest_rate2 = curr_states.nominal_interest_rate2.clone();
        self.non_performing_date = curr_states.non_performing_date.clone();
        self.notional_principal = curr_states.notional_principal.clone();
        self.notional_principal2 = curr_states.notional_principal2.clone();
        self.notional_scaling_multiplier = curr_states.notional_scaling_multiplier.clone();
        self.status_date = curr_states.status_date.clone();
        self.maturity_date = curr_states.maturity_date.clone();
        self.termination_date = curr_states.termination_date.clone();
        self.boundary_crossed_flag = curr_states.boundary_crossed_flag.clone();
        self.boundary_monitoring_flag = curr_states.boundary_monitoring_flag.clone();
        self.boundary_leg1_active_flag = curr_states.boundary_leg1_active_flag.clone();
        self.boundary_leg2_active_flag = curr_states.boundary_leg2_active_flag.clone();
        self.last_interest_period = curr_states.last_interest_period.clone();

    }
}
