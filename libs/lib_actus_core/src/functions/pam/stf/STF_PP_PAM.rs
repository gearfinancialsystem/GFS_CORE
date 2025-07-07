use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_PP_PAM;

impl TraitStateTransitionFunction for STF_PP_PAM {
    fn eval(
        &self,
        time: &IsoDatetime, 
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) {

        let status_date = states.status_date.expect("status date should be some");
        let nominal_interest_rate = states.nominal_interest_rate.expect("nominalInterestRate should be some");
        let notional_principal = states.notional_principal.expect("notionalPrincipal should be some");
        let fee_rate = model.fee_rate.expect("fee rate should be some");


        // Calculate time from the last event
        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date),
            time_adjuster.shift_sc(&time),
        );

        states.accrued_interest = states.accrued_interest.map(|mut accrued_interest| {
            accrued_interest += nominal_interest_rate * notional_principal * time_from_last_event;
            accrued_interest
        });

        states.fee_accrued = states.fee_accrued.map(|mut fee_accrued| {
            fee_accrued += fee_rate * notional_principal * time_from_last_event;
            fee_accrued
        });

        states.notional_principal = states.notional_principal.map(|mut notional_princial| {
            notional_princial -= 1.0 * notional_principal;
            notional_princial
        });
        
        states.status_date = Some(*time);


    }
}
