use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
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
        _risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) {

        let status_date = states.status_date.as_ref().expect("status date should be some");
        let nominal_interest_rate = states.nominal_interest_rate.as_ref().expect("nominalInterestRate should be some");
        let notional_principal = states.notional_principal.as_ref().expect("notionalPrincipal should be some");
        let fee_rate = model.fee_rate.as_ref().expect("fee rate should be some");


        // Calculate time from the last event
        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.value()),
            time_adjuster.shift_sc(&time),
        );

        states.accrued_interest = states.accrued_interest.clone().map(|mut accrued_interest| {
            accrued_interest += nominal_interest_rate.value() * notional_principal.value() * time_from_last_event;
            accrued_interest
        });

        states.fee_accrued = states.fee_accrued.clone().map(|mut fee_accrued| {
            fee_accrued += fee_rate.value() * notional_principal.value() * time_from_last_event;
            fee_accrued
        });

        states.notional_principal = states.notional_principal.clone().map(|mut notional_princial| {
            notional_princial -= 1.0 * notional_principal.value();
            notional_princial
        });
        
        states.status_date = Some(StatusDate::from(*time));


    }
}
