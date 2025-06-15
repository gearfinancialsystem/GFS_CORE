use crate::traits::StateTransitionFunctionTrait::StateTransitionFunctionTrait;
use crate::contracts::ContractModel::ContractModel;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::states::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;

#[allow(non_camel_case_types)]
pub struct STF_MD_PAM;

impl StateTransitionFunctionTrait for STF_MD_PAM {
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
        // Set state values to zero at maturity
        new_states.notionalPrincipal = Some(Box::new(0.0));
        new_states.accruedInterest = Some(Box::new(0.0));
        new_states.feeAccrued = Some(Box::new(0.0));
        
        // Update the status date
        new_states.statusDate = time;

        // Return a copy of the updated state space
        new_states
    }
}
