use lib_actus_terms::ContractTerms::{ContractTerms};
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_events::traits::TraitPayOffFunction::TraitPayOffFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;

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
        states: &StatesSpace,
        model: &ContractTerms,
        risk_factor_model: Option<&dyn TraitRiskFactorModel>,
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