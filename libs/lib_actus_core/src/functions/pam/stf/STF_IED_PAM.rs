use crate::traits::StateTransitionFunctionTrait::StateTransitionFunctionTrait;
use crate::contracts::ContractModel::ContractModel;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::states::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;


#[allow(non_camel_case_types)]
pub struct STF_IED_PAM;

impl StateTransitionFunctionTrait for STF_IED_PAM {
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
        // update state space

        // Modify the value inside the Option<Box<f64>>
        if let Some(value) = new_states.notionalPrincipal.as_deref_mut() {
            *value += model.ContractRole.role_sign() * model.NotionalPrincipal.unwrap(); // Dereference just once due to as_deref_mut
        }
        
        new_states.nominalInterestRate = model.NominalInterestRate;
        new_states.statusDate = time;

        // if cycle anchor date of interest payment prior to IED, then update nominal accrued accordingly
        // a refaire
        let cycle_anchor_date = &model.CycleAnchorDateOfInterestPayment;
         
        if cycle_anchor_date < &model.InitialExchangeDate {
            // Modify the value inside the Option<Box<f64>>
            if let Some(value) = new_states.accruedInterest.as_deref_mut() {
                *value += states.notionalPrincipal.as_deref().unwrap() * states.nominalInterestRate
                    * day_counter.day_count_fraction(
                    time_adjuster.shift_bd(&cycle_anchor_date),
                    time_adjuster.shift_bd(&time),
                ); // Dereference just once due to as_deref_mut
            }
        }
        


        // return post-event states
        new_states
    }
}
