use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_PP_PAM;

impl TraitPayOffFunction for POF_PP_PAM {
    fn eval(
        &self,
        _time: &IsoDatetime,
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayConvention,
    ) -> f64 {

            assert!(model.notionalScalingMultiplier.is_some(), "nominal interest rate should always be Some");
            assert!(states.notionalPrincipal.is_some(), "notional principal should always be Some");
            assert!(model.contractRole.is_some(), "contractRole rate should always be Some");
            assert!(model.objectCodeOfPrepaymentModel.is_some(), "objectCode of prepayment model should always be Some");

            let notional_scaling_multiplier = model.notionalScalingMultiplier.unwrap();
            let notional_principal = states.notionalPrincipal.unwrap();
            let contract_role = model.contractRole.as_ref().unwrap();
            let objectCodeOfPrepaymentModel = model.objectCodeOfPrepaymentModel.as_ref().unwrap();

            1.0 * contract_role.role_sign() * 1.0 * notional_principal

    }
}
