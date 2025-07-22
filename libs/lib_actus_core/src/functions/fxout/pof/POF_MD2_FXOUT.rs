use crate::attributes::ContractTerms::ContractTerms;

use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::external::RiskFactorModels::RiskFactors;

#[allow(non_camel_case_types)]
pub struct POF_MD2_FXOUT;

impl TraitPayOffFunction for POF_MD2_FXOUT {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StateSpace,
        model: &ContractTerms,
        risk_factor_model: &RiskFactors,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let contract_role = model.contract_role.clone().expect("contract role should always exist");
        let contract_role_sign = contract_role.role_sign();
        let notional_principal_2 = model.notional_principal2.clone().expect("notionalPrincipal2 should always exist");

        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            model,
            time,
            states
        );

        settlement_currency_fx_rate * contract_role_sign * -1.0 * notional_principal_2.value()
    }
}
