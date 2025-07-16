use crate::attributes::ContractModel::ContractModel;

use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;
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
        time: &IsoDatetime,
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &DataObserver,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            model,
            time,
            states
        );
        let price_at_termination = model.price_at_termination_date.clone().expect("No price at termination");
        let accrued_interest = states.accrued_interest.clone().expect("No accrued interest");

        settlement_currency_fx_rate * price_at_termination.value() + accrued_interest.value()
    }
}