use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_PRD_FXOUT;

impl TraitPayOffFunction for POF_PRD_FXOUT {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let contract_role = model.contract_role.clone().expect("contract role should always exist");
        let contract_role_sign = contract_role.role_sign();
        let price_at_purchase_date = model.price_at_purchase_date.clone().expect("priceAtPurchaseDate should always exist");

        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            model,
            time,
            states
        );

        settlement_currency_fx_rate * contract_role_sign * -1.0 * price_at_purchase_date.value()
    }
}
