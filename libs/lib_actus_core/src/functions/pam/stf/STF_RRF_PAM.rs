use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_RRF_PAM;

impl TraitStateTransitionFunction for STF_RRF_PAM {
    fn eval(
        &self,
        time: &IsoDatetime, 
        states: &mut StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        
        let status_date = states.status_date.as_ref().expect("status date should always be some");
        let nominal_interest_rate = states.nominal_interest_rate.as_ref().expect("nominalInterestRate should always be None");
        let notional_principal = states.notional_principal.as_ref().expect("notionalPrincipal should always be None");
        let fee_rate = model.fee_rate.as_ref().expect("fee rate should always be None");
        let next_reset_rate = model.next_reset_rate.as_ref().expect("next_reset_rate should always be None");

        let time_from_last_event = day_counter.day_count_fraction(time_adjuster.shift_sc(&status_date.value()),
                                                                  time_adjuster.shift_sc(time));
        
        states.accrued_interest = states.accrued_interest.clone().map(|mut accrued_interest| {
            accrued_interest += nominal_interest_rate.value() * notional_principal.value() * time_from_last_event;
            accrued_interest
        });

        states.fee_accrued = states.fee_accrued.clone().map(|mut fee_accrued| {
            fee_accrued += fee_rate.value() * notional_principal.value() * time_from_last_event;
            fee_accrued
        });

        states.nominal_interest_rate = NominalInterestRate::new(next_reset_rate.value()).ok(); //Some(next_reset_rate);
        states.status_date = Some(StatusDate::from(*time));


    }
}
