use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;

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
        time_adjuster: &BusinessDayConvention,
    ) {

        let status_date = states.statusDate.expect("status date should be some");
        let accrued_interest = states.accruedInterest.expect("accrued interest should be some");
        let nominal_interest_rate = states.nominalInterestRate.expect("nominalInterestRate should be some");
        let notional_principal = states.notionalPrincipal.expect("notionalPrincipal should be some");
        let fee_rate = model.feeRate.expect("fee rate should be some");


        // Calculate time from the last event
        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_bd(&status_date),
            time_adjuster.shift_bd(&time),
        );

        states.accruedInterest = states.accruedInterest.map(|mut accrued_interest| {
            accrued_interest += nominal_interest_rate * notional_principal * time_from_last_event;
            accrued_interest
        });

        states.feeAccrued = states.feeAccrued.map(|mut fee_accrued| {
            fee_accrued += fee_rate * notional_principal * time_from_last_event;
            fee_accrued
        });

        states.notionalPrincipal = states.notionalPrincipal.map(|mut notional_princial| {
            notional_princial -= 1.0 * notional_principal;
            notional_princial
        });
        
        states.statusDate = Some(*time);


    }
}
