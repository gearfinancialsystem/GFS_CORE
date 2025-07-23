use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use lib_actus_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;

use lib_actus_events::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_terms::traits::TraitOptionExt::TraitOptionExt;

#[allow(non_camel_case_types)]
pub struct STF_IED_SWPPV;

impl TraitStateTransitionFunction for STF_IED_SWPPV {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        model: &ContractTerms,
        _risk_factor_model: Option<&dyn TraitRiskFactorModel>,
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
