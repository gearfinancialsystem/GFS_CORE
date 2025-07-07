use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::state_space::StateSpace::StateSpace;
use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::types::IsoDatetime::IsoDatetime;


#[allow(non_camel_case_types)]
pub struct STF_CD_LAM;

impl TraitStateTransitionFunction for STF_CD_LAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    )  {
        // Create a mutable copy of the states to update
        let status_date = states.status_date.expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.expect("nominalInterestRate should always be Some");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.expect("interestCalculationBaseAmount should always be Some");
        let notional_principal = states.notional_principal.expect("notionalPrincipal should always be Some");
        let fee_rate = model.fee_rate.clone().expect("fee rate should always be Some");
        
        // Update state space
        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date),
            time_adjuster.shift_sc(time),
        );
        states.accrued_interest = states.accrued_interest.map(|mut accrued_interest| {
            accrued_interest += nominal_interest_rate * interest_calculation_base_amount * time_from_last_event;
            accrued_interest
        });
        
        states.fee_accrued = states.fee_accrued.map(|mut fee_accrued| {
            fee_accrued += fee_rate * notional_principal * time_from_last_event;
            fee_accrued
        });
        
        states.contract_performance = Some(ContractPerformance::new("DF").expect("ok cp")  );
        states.status_date = Some(*time);

    }
}
