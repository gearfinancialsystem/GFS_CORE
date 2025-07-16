use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use chrono::NaiveDateTime;
use crate::attributes::ContractModel::ContractModel;
use crate::state_space::StateSpace::StateSpace;
use crate::util_tests::essai_data_observer::DataObserver;

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
        time: &NaiveDateTime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &DataObserver,//&RiskFactorModel,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) ; // -> StateSpace
}
