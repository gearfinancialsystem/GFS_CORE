use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::attributes::ContractReference::ContractReference;
#[allow(non_camel_case_types)]
pub struct POF_IPFloat_SWPPV;

impl TraitPayOffFunction for POF_IPFloat_SWPPV {
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

        settlement_currency_fx_rate * (
            states.accrued_interest2.clone().unwrap().value() +
                (-1.0) *
                    states.last_interest_period.clone().unwrap() *
                    states.nominal_interest_rate.clone().unwrap().value() *
                    states.notional_principal.clone().unwrap().value()
        )
    }
}
