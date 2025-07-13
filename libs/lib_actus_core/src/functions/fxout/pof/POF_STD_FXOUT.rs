use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_STD_FXOUT;

impl TraitPayOffFunction for POF_STD_FXOUT {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let contract_role = model.contract_role.as_ref().expect("contract role should always exist");
        let contract_role_sign = contract_role.role_sign();
        let notional_principal = model.notional_principal.clone().expect("notionalPrincipal should always exist");
        let notional_principal_2 = model.notional_principal2.clone().expect("notionalPrincipal2 should always exist");
        

        let strings = vec![
                            model.currency2.clone().unwrap().to_currency(),
                            model.currency.clone().unwrap()
        ];

        let str_slices: Vec<String> = strings.iter().map(|s| s.value().clone().to_string()).collect();
        let joined = str_slices.join(" ");

        let risk_factor_placeholder = risk_factor_model.state_at(&joined, time, states, model,true).unwrap();

        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            model,
            time,
            states
        );

        let payoff = settlement_currency_fx_rate * contract_role_sign * (notional_principal.value() - risk_factor_placeholder * notional_principal_2.value());

        payoff
    }
}
