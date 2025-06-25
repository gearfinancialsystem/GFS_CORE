use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_IED_CLM;

impl TraitPayOffFunction for POF_IED_CLM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let contract_role = model.contractRole.as_ref().expect("contract role should always exist");
        let notional_principal = model.notionalPrincipal.expect("notionalPrincipal should always exist");
        let settlement_currency_fx_rate = 1.0; // Remplacer par 1.0 comme demandé

        settlement_currency_fx_rate
            * contract_role.role_sign()
            * (-1.0)
            * notional_principal
    }
}
