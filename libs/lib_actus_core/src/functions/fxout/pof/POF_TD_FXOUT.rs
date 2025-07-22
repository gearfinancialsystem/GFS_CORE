use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_events::traits::TraitPayOffFunction::TraitPayOffFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;

#[allow(non_camel_case_types)]
pub struct POF_TD_FXOUT;

impl TraitPayOffFunction for POF_TD_FXOUT {
    fn eval(
        &self,
        _time: &IsoDatetime,
        _states: &StatesSpace,
        model: &ContractTerms,
        _risk_factor_model: Option<&dyn TraitRiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let price_at_termination_date = model.price_at_termination_date.clone().expect("priceAtTerminationDate should always exist");

        1.0 * price_at_termination_date.value()
    }
}
