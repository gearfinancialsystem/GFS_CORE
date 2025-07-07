use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_IED_SWPPV;

impl TraitStateTransitionFunction for STF_IED_SWPPV {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        let contract_role = model.contract_role.as_ref().expect("contractRole should always be Some");
        let role_sign = contract_role.role_sign();

        let notional_principal = model.notional_principal.unwrap_or(0.0);
        states.notional_principal = Some(role_sign * notional_principal);

        let nominal_interest_rate = model.nominal_interest_rate2.unwrap_or(0.0);
        states.nominal_interest_rate = Some(nominal_interest_rate);

        states.status_date = Some(*time);
    }
}
