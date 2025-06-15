use crate::traits::StateTransitionFunctionTrait::StateTransitionFunctionTrait;
use crate::contracts::ContractModel::ContractModel;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::states::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;

#[allow(non_camel_case_types)]
pub struct STF_RR_PAM;

impl StateTransitionFunctionTrait for STF_RR_PAM {
    fn eval(
        &self,
        time: IsoDatetime, 
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayConvention,
    ) -> StateSpace {

        let mut new_states: StateSpace = states.copy_state_space();

        // Compute new rate
        let base_rate = 0.0; // risk_factor_model.state_at(model.marketObjectCodeOfRateReset, &time, states, model);
        
        let mut rate = base_rate * model.RateMultiplier.unwrap() + model.RateSpread.unwrap();
        let mut delta_rate = rate - states.nominalInterestRate;

        // Apply period cap/floor
        delta_rate = delta_rate.clamp(model.PeriodFloor.unwrap(), model.PeriodCap.unwrap());
        
        rate = states.nominalInterestRate + delta_rate;

        // Apply life cap/floor
        rate = rate.clamp(model.LifeFloor.unwrap(), model.LifeCap.unwrap());

        // Update state space
        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_bd(&states.statusDate),
            time_adjuster.shift_bd(&time),
        );
        
        // Update accrued interest and fee accrued
        if let Some(value) = new_states.accruedInterest.as_deref_mut() {
            *value += states.nominalInterestRate * states.notionalPrincipal.as_deref().unwrap() * time_from_last_event; // Dereference just once due to as_deref_mut
        }
        new_states.nominalInterestRate = rate;
        new_states.statusDate = time;

        // Return a copy of the updated state space
        new_states
    }
}
