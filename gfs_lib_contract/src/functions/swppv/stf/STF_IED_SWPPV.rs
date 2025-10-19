use std::sync::Arc;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use gfs_lib_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use gfs_lib_terms::traits::TraitOptionExt::TraitOptionExt;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::traits::TraitExternalData::TraitExternalData;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct STF_IED_SWPPV;

impl TraitStateTransitionFunction for STF_IED_SWPPV {
    fn new() -> Self {
        Self {}
    }
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &mut StatesSpace,
        contract_terms: &ContractTerms,
        _contract_structure: &Option<RelatedContracts>,
        _risk_factor_external_data: &Option<Arc<dyn TraitExternalData>>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        let contract_role = contract_terms.contract_role.as_ref().expect("contractRole should always be Some");
        let role_sign = contract_role.role_sign();

        let notional_principal = contract_terms.notional_principal.clone().itself_or(0.0);
        states.notional_principal = NotionalPrincipal::new(role_sign * notional_principal.value()).ok();

        let nominal_interest_rate = contract_terms.nominal_interest_rate2.clone().itself_or(0.0);
        states.nominal_interest_rate = NominalInterestRate::new(nominal_interest_rate.value()).ok();

        states.status_date = StatusDate::new(time.value()).ok();
    }
}
