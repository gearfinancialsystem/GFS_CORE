use crate::traits::StateTransitionFunctionTrait::StateTransitionFunctionTrait;
use crate::contracts::ContractModel::ContractModel;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::states::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_fees::FeeBasis::FeeBasis;
use crate::terms::grp_fees::fee_basis::N::N;
#[allow(non_camel_case_types)]
pub struct STF_IP_PAM;

impl StateTransitionFunctionTrait for STF_IP_PAM {
    fn eval(
        &self,
        time: IsoDatetime, 
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayConvention,
    ) -> StateSpace {
        // Reset accrued interest
        let mut new_states: StateSpace = states.copy_state_space(); 
        
        new_states.accruedInterest = Some(Box::new(0.0));

        // Update fee-accrued
        if *model.FeeBasis == FeeBasis::N(N) {
            let time_fraction = day_counter.day_count_fraction(
                time_adjuster.shift_bd(&states.statusDate),
                time_adjuster.shift_bd(&time),
            );

            // Modify the value inside the Option<Box<f64>>
            if let Some(value) = new_states.feeAccrued.as_deref_mut() {
                *value += time_fraction * model.FeeRate.unwrap() * states.notionalPrincipal.as_deref().unwrap(); // Dereference just once due to as_deref_mut
            }
        
        } 
        // Further processing for FeeBasis cases that aren't "N"
        // Commented out: If additional logic is needed, it can be expanded here

        // Update the status date
        new_states.statusDate = time;

        // Return a copy of the updated state space
        new_states
    }
}
