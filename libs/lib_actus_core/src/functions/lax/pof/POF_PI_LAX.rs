use std::arch::x86_64::_mm256_set_epi16;
use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_events::traits::TraitPayOffFunction::TraitPayOffFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;
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
        states: &StatesSpace,
        model: &ContractTerms,
        risk_factor_model: Option<&dyn TraitRiskFactorModel>,
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
