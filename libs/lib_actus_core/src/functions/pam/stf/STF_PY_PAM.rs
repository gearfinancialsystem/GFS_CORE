use crate::traits::StateTransitionFunctionTrait::StateTransitionFunctionTrait;
use crate::contracts::ContractModel::ContractModel;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::states::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;

#[allow(non_camel_case_types)]
pub struct STF_PY_PAM;
impl StateTransitionFunctionTrait for STF_PY_PAM {
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

        // Calculate time from the last event
        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_bd(&states.statusDate),
            time_adjuster.shift_bd(&time),
        );

        // Update accrued interest and fee accrued
        if let Some(value) = new_states.accruedInterest.as_deref_mut() {
            *value += states.nominalInterestRate * states.notionalPrincipal.as_deref().unwrap() * time_from_last_event; // Dereference just once due to as_deref_mut
        }
        
        if let Some(value) = new_states.feeAccrued.as_deref_mut() {
            *value += model.FeeRate.unwrap() * states.notionalPrincipal.as_deref().unwrap() * time_from_last_event; // Dereference just once due to as_deref_mut
        }
        
        // Update the status date
        new_states.statusDate = time;

        // Return a copy of the updated state space
        new_states
    }
}
