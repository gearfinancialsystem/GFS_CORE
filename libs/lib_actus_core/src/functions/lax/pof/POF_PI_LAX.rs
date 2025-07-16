use std::arch::x86_64::_mm256_set_epi16;
use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_PI_LAX {
    pr_payment: f64,
}

impl POF_PI_LAX {
    pub fn new(pr_payment: f64) -> Self {
        POF_PI_LAX { pr_payment }
    }
}

impl TraitPayOffFunction for POF_PI_LAX {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        
        let contract_role = model.contract_role.clone().expect("contract role should always exist");
        let notional_scaling_multiplier = model.notional_scaling_multiplier.clone().expect("notionalScalingMultiplier should always exist");
        let notional_principal = states.notional_principal.clone().expect("notionalPrincipal should always exist");

        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            model,
            time,
            states
        );
        
        let redemption = self.pr_payment - contract_role.role_sign()
            * f64::max(0.0, self.pr_payment.abs() - notional_principal.value().abs());

        settlement_currency_fx_rate * -1.0 * contract_role.role_sign() * notional_scaling_multiplier.value() * redemption
    }
}
