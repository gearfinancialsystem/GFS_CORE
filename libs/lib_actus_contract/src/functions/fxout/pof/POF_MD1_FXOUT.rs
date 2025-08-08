use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::external::RiskFactorModel::RiskFactorModel;

#[allow(non_camel_case_types)]
pub struct POF_MD1_FXOUT;

impl TraitPayOffFunction for POF_MD1_FXOUT {
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &StatesSpace,
        contract_terms: &ContractTerms,
contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<impl TraitRiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let contract_role = contract_terms.contract_role.clone().expect("contract role should always exist");
        let contract_role_sign = contract_role.role_sign();
        let notional_principal = contract_terms.notional_principal.clone().expect("notionalPrincipal should always exist");

        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            contract_terms,
            time,
            states
        );
        settlement_currency_fx_rate * contract_role_sign * notional_principal.value()
    }
}
