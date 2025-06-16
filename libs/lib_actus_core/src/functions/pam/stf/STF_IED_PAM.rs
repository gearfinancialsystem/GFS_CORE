use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_IED_PAM;

impl TraitStateTransitionFunction for STF_IED_PAM {
    fn eval(
        &self,
        time: &IsoDatetime, 
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayConvention,
    )  {

        // let mut new_states: StateSpace = states.copy_state_space();
        // update state space

        // Modify the value inside the Option<Box<f64>>
        states.notionalPrincipal = match (states.notionalPrincipal, &model.contractRole) {
            (Some(a), Some(b)) => Some(a + (b.role_sign() * a)),
            (accrued_interest, _) => accrued_interest,
        };

        states.nominalInterestRate = model.nominalInterestRate;

        states.statusDate = Some(*time);

        // if cycle anchor date of interest payment prior to IED, then update nominal accrued accordingly
        // a refaire
        let cycle_anchor_date = model.cycleAnchorDateOfInterestPayment.unwrap();



        if cycle_anchor_date < model.initialExchangeDate.unwrap() {
            
            states.accruedInterest = match (states.accruedInterest, states.nominalInterestRate, states.notionalPrincipal) {
                (Some(a), Some(b), Some(c)) => Some(a + (b * c * 
                    day_counter.day_count_fraction(time_adjuster.shift_bd(&cycle_anchor_date), 
                                                   time_adjuster.shift_bd(&time)))),
                (accrued_interest, _, _) => accrued_interest,
            };
        }
        
        
    }
}
