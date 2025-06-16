use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_RRF_PAM;

impl TraitStateTransitionFunction for STF_RRF_PAM {
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
        // Calculate time from the last event
        let timeFromLastEvent = day_counter.day_count_fraction(
            time_adjuster.shift_bd(&states.statusDate.unwrap()),
            time_adjuster.shift_bd(&time),
        );

        states.accruedInterest = match (states.accruedInterest, states.nominalInterestRate, states.notionalPrincipal) {
            (Some(a), Some(b), Some(c)) => Some(a + (b * c * timeFromLastEvent)),
            (accrued_interest, _, _) => accrued_interest,
        };

        states.feeAccrued = match (states.feeAccrued, model.feeRate, states.notionalPrincipal) {
            (Some(a), Some(b), Some(c)) => Some(a + (b * c * timeFromLastEvent)),
            (fee_accrued, _, _) => fee_accrued,
        };

        // Set the nominal interest rate to the next reset rate
        
        states.nominalInterestRate = model.nextResetRate;
        states.statusDate = Some(*time)


    }
}
