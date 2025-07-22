use crate::attributes::ContractTerms::ContractModel;

use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use lib_actus_events::traits::TraitPayOffFunction::TraitPayOffFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;

#[allow(non_camel_case_types)]
pub struct POF_IP_CLM;

impl TraitPayOffFunction for POF_IP_CLM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StatesSpace,
        model: &ContractModel,
        risk_factor_model: &DataObserver,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            model,
            time,
            states
        );
        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&states.status_date.clone().unwrap().value()),
            time_adjuster.shift_sc(time)
        );

        settlement_currency_fx_rate * (
            states.accrued_interest.clone().unwrap().value() +
                time_from_last_event *
                    states.nominal_interest_rate.clone().unwrap().value() *
                    states.notional_principal.clone().unwrap().value()
        )
    }
}
