use crate::terms::grp_notional_principal::scaling_effect::Ino::INO;
use crate::terms::grp_notional_principal::scaling_effect::Ioo::IOO;
use crate::terms::grp_notional_principal::scaling_effect::Ono::ONO;
use crate::terms::grp_notional_principal::ScalingEffect;
use crate::traits::StateTransitionFunctionTrait::StateTransitionFunctionTrait;
use crate::contracts::ContractModel::ContractModel;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::states::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;

#[allow(non_camel_case_types)]
pub struct STF_SC_PAM;

impl StateTransitionFunctionTrait for STF_SC_PAM {
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
        
        // Calculate scaling multiplier
        let scaling_index = &model.ScalingIndexAtContractDealDate;
        let scaling_multiplier = 1.0; // risk_factor_model.state_at(model.MarketObjectCodeOfScalingIndex, &time, states, model) / scaling_index;

        // Apply scaling effect to interest or notional scaling multipliers
        let scaling_effect = &model.ScalingEffect;
        if *scaling_effect.as_ref() == ScalingEffect::ScalingEffect::INO(INO) || *scaling_effect.as_ref() == ScalingEffect::ScalingEffect::IOO(IOO) {
            new_states.interestScalingMultiplier = scaling_multiplier;
        }
        if *scaling_effect.as_ref() == ScalingEffect::ScalingEffect::INO(INO) || *scaling_effect.as_ref() == ScalingEffect::ScalingEffect::ONO(ONO) {
            new_states.notionalScalingMultiplier = scaling_multiplier;
        }

        // Update the status date
        new_states.statusDate = time;

        // Return a copy of the updated state space
        new_states
    }
}
