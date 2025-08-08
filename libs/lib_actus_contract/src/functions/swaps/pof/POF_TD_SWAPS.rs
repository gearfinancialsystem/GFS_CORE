use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::attributes::ContractTerms::{ContractTerms};
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::attributes::ContractReference::ContractReference;
use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;

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
        time: &PhantomIsoDatetimeW,
        states: &StatesSpace,
        contract_terms: &ContractTerms,
        _contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<impl TraitRiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            contract_terms,
            time,
            states
        );
        let price_at_termination = contract_terms.price_at_termination_date.clone().expect("No price at termination");
        let accrued_interest = states.accrued_interest.clone().expect("No accrued interest");

        settlement_currency_fx_rate * price_at_termination.value() + accrued_interest.value()
    }
}