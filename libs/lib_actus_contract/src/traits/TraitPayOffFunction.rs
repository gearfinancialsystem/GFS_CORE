use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ContractTerms::ContractTerms;
// use crate::external::RiskFactorModel::RiskFactorModel;
use crate::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_types::types::IsoDatetime::IsoDatetime;

use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;

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
        time: &PhantomIsoDatetimeW,
        states: &StatesSpace,
        contract_terms: &ContractTerms,
        contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<impl TraitRiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64;
}
