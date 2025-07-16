use crate::attributes::ContractModel::ContractModel;

use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::traits::TraitOptionExt::TraitOptionExt;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;
#[allow(non_camel_case_types)]
pub struct STF_IED_SWPPV;

impl TraitStateTransitionFunction for STF_IED_SWPPV {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        _risk_factor_model: &DataObserver,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        let contract_role = model.contract_role.as_ref().expect("contractRole should always be Some");
        let role_sign = contract_role.role_sign();

        let notional_principal = model.notional_principal.clone().itself_or(0.0);
        states.notional_principal = NotionalPrincipal::new(role_sign * notional_principal.value()).ok();

        let nominal_interest_rate = model.nominal_interest_rate2.clone().itself_or(0.0);
        states.nominal_interest_rate = NominalInterestRate::new(nominal_interest_rate.value()).ok();

        states.status_date = Some(StatusDate::from(*time));
    }
}
