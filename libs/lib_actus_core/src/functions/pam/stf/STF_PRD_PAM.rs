use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_PRD_PAM;

impl TraitStateTransitionFunction for STF_PRD_PAM {
    fn eval(
        &self,
        time: &IsoDatetime, 
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayConvention,
    ) {

        //let mut new_states: StateSpace = states.copy_state_space();

        // Calculate time from the last event
        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_bd(&states.statusDate.unwrap()),
            time_adjuster.shift_bd(&time),
        );

        // Update accrued interest and fee accrued
        states.accruedInterest = match (states.accruedInterest, states.nominalInterestRate, states.notionalPrincipal, time_from_last_event) {
            (Some(a), Some(b), Some(c), d) => Some(a + (b * c * d)),
            (accruedInterest, _, _, _) => accruedInterest,
        };
        
        states.feeAccrued = match (states.feeAccrued, model.feeRate, states.notionalPrincipal, time_from_last_event) {
            (Some(a), Some(b), Some(c), d) => Some(a + (b * c * d)),
            (feeAccrued, _, _, _) => feeAccrued,
        };

        // Update the status date
        states.statusDate = Some(*time)

    }
}
