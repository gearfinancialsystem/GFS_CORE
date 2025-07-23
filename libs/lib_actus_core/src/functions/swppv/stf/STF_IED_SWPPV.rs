use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;

use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::traits::TraitOptionExt::TraitOptionExt;
use crate::attributes::ContractReference::ContractReference;
#[allow(non_camel_case_types)]
pub struct STF_IED_SWPPV;

impl TraitStateTransitionFunction for STF_IED_SWPPV {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        contract_terms: &ContractTerms,
contract_structure: &Option<Vec<ContractReference>>,
        _risk_factor_model: &Option<RiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        let contract_role = contract_terms.contract_role.as_ref().expect("contractRole should always be Some");
        let role_sign = contract_role.role_sign();

        let notional_principal = contract_terms.notional_principal.clone().itself_or(0.0);
        states.notional_principal = NotionalPrincipal::new(role_sign * notional_principal.value()).ok();

        let nominal_interest_rate = contract_terms.nominal_interest_rate2.clone().itself_or(0.0);
        states.nominal_interest_rate = NominalInterestRate::new(nominal_interest_rate.value()).ok();

        states.status_date = Some(StatusDate::from(*time));
    }
}
