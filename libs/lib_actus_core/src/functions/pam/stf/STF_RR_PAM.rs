use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_RR_PAM;

impl TraitStateTransitionFunction for STF_RR_PAM {
    fn eval(
        &self,
        time: &IsoDatetime, 
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayConvention,
    ) {

        // let mut new_states: StateSpace = states.copy_state_space();

        // Compute new rate
        let base_rate = 0.0; // risk_factor_model.state_at(model.marketObjectCodeOfRateReset, &time, states, model);
        
        let mut rate = base_rate * model.rateMultiplier.unwrap() + model.rateSpread.unwrap();
        let mut delta_rate = rate - states.nominalInterestRate.unwrap();

        // Apply period cap/floor
        delta_rate = delta_rate.clamp(model.periodFloor.unwrap(), model.periodCap.unwrap());
        
        rate = states.nominalInterestRate.unwrap() + delta_rate;

        // Apply life cap/floor
        rate = rate.clamp(model.lifeFloor.unwrap(), model.lifeCap.unwrap());

        // Update state space
        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_bd(&states.statusDate.unwrap()),
            time_adjuster.shift_bd(&time),
        );
        
        // Update accrued interest and fee accrued
        states.accruedInterest = match (states.accruedInterest, states.nominalInterestRate, states.notionalPrincipal, time_from_last_event) {
            (Some(a), Some(b), Some(c), d) => Some(a + (b * c * d)),
            (accrued_interest, _, _, _) => accrued_interest,
        };

        states.nominalInterestRate = Some(rate);
        states.statusDate = Some(*time);


    }
}
