use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_TD_SWAPS;

impl POF_TD_SWAPS {
    pub fn new() -> Self {
        POF_TD_SWAPS
    }
}

impl TraitPayOffFunction for POF_TD_SWAPS {
    fn eval(
        &self,
        _time: &IsoDatetime,
        states: &StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let fx_rate = 1.0;
        let price_at_termination = model.priceAtTerminationDate.expect("No price at termination");
        let accrued_interest = states.accruedInterest.expect("No accrued interest");
        fx_rate * price_at_termination + accrued_interest
    }
}