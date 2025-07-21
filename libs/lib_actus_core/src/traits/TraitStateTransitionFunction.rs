use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use chrono::NaiveDateTime;
use crate::attributes::ContractTerms::{ContractTerms};
use crate::external::RiskFactors::RiskFactors;
use crate::state_space::StateSpace::StateSpace;
use crate::types::IsoDatetime::IsoDatetime;


#[allow(non_snake_case)]
pub trait TraitStateTransitionFunction {
    /// Evaluate the function.
    ///
    /// * `time` - The schedule time of this particular event.
    /// * `states` - The current state of the contract.
    /// * `model` - The model containing parsed contract attributes.
    /// * `risk_factor_model` - An external market model.
    /// * `day_counter` - The day count convention used to calculate day count fractions.
    /// * `time_adjuster` - The business day convention used to shift the schedule time.
    
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractTerms,
        risk_factor_model: &RiskFactors,//&RiskFactorModel,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) ; // -> StateSpace
}
