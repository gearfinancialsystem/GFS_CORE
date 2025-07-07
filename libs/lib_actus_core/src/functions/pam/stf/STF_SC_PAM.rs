use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_SC_PAM;

impl TraitStateTransitionFunction for STF_SC_PAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) { // ->StateSpace

        let status_date = states.status_date.expect("status date should always be some");
        let nominal_interest_rate = states.nominal_interest_rate.expect("nominalInterestRate should always be None");
        let notional_principal = states.notional_principal.expect("notionalPrincipal should always be None");
        let fee_rate = model.fee_rate.expect("fee rate should always be None");
        let scaling_effect = model.scalingEffect.as_ref().expect("scalingEffect should always be None");
        
        let time_from_last_event = day_counter.day_count_fraction(time_adjuster.shift_sc(&status_date),
                                                                  time_adjuster.shift_sc(time));

        states.accrued_interest = states.accrued_interest.map(|mut accrued_interest| {
            accrued_interest += nominal_interest_rate * notional_principal * time_from_last_event;
            accrued_interest
        });

        states.fee_accrued = states.fee_accrued.map(|mut fee_accrued| {
            fee_accrued += fee_rate * notional_principal * time_from_last_event;
            fee_accrued
        });
        
        let scaling_multiplier = 1.0; // implementer risk factor
        
        if scaling_effect.to_string().contains("I") {
            states.interest_scaling_multiplier = Some(scaling_multiplier);
        }
        if scaling_effect.to_string().contains("N") {
            states.notional_scaling_multiplier = Some(scaling_multiplier);
        }
        
        states.status_date = Some(*time);
        
    }
}
