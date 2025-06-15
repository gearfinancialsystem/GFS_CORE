use crate::attributes::ContractModel::ContractModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::types::isoDatetime::IsoDatetime;

pub trait TraitPayOffFunction {
    /// Evaluate the function.
    ///
    /// * `time` - The schedule time of this particular event.
    /// * `states` - The current state of the contract.
    /// * `model` - The model containing parsed contract attributes.
    /// * `risk_factor_model` - An external market model.
    /// * `day_counter` - The day count convention used to calculate day count fractions.
    /// * `time_adjuster` - The business day convention used to shift the schedule time.
    ///
    /// Returns the payoff amount as a `f64`.
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayConvention,
    ) -> f64;
}
