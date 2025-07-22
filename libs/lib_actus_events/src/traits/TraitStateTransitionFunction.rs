
use chrono::NaiveDateTime;
use lib_actus_terms::ContractTerms::{ContractTerms};

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_types::types::IsoDatetime::IsoDatetime;

use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;

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
        states: &mut StatesSpace,
        model: &ContractTerms,
        risk_factor_model: Option<&dyn TraitRiskFactorModel>, //&RiskFactorModel,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) ; // -> StateSpace
}
