use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_STD_FXOUT;

impl TraitPayOffFunction for POF_STD_FXOUT {
    fn eval(
        &self,
        _time: &IsoDatetime,
        _states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let contract_role = model.contractRole.as_ref().expect("contract role should always exist");
        let contract_role_sign = contract_role.role_sign();
        let notional_principal = model.notionalPrincipal.expect("notionalPrincipal should always exist");
        let notional_principal_2 = model.notionalPrincipal2.expect("notionalPrincipal2 should always exist");

        // Placeholder for the risk factor calculation
        let risk_factor_placeholder = 1.0; // Replace with actual logic later

        let payoff = 1.0 * contract_role_sign * (notional_principal - risk_factor_placeholder * notional_principal_2);

        payoff
    }
}
