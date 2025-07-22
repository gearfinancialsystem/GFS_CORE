use crate::attributes::ContractTerms::ContractModel;

use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_events::traits::TraitPayOffFunction::TraitPayOffFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;
#[allow(non_camel_case_types)]
pub struct POF_IPFloat_SWPPV;

impl TraitPayOffFunction for POF_IPFloat_SWPPV {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StatesSpace,
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

        settlement_currency_fx_rate * (
            states.accrued_interest2.clone().unwrap().value() +
                (-1.0) *
                    states.last_interest_period.clone().unwrap() *
                    states.nominal_interest_rate.clone().unwrap().value() *
                    states.notional_principal.clone().unwrap().value()
        )
    }
}
