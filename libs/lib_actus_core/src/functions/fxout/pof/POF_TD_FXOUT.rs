use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_TD_FXOUT;

impl TraitPayOffFunction for POF_TD_FXOUT {
    fn eval(
        &self,
        _time: &IsoDatetime,
        _states: &StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let price_at_termination_date = model.price_at_termination_date.clone().expect("priceAtTerminationDate should always exist");

        1.0 * price_at_termination_date.value()
    }
}
