use std::sync::Arc;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use gfs_lib_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct STF_IP_SWPPV;

impl TraitStateTransitionFunction for STF_IP_SWPPV {
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
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        states.accrued_interest = AccruedInterest::new(0.0).ok();
        states.status_date = StatusDate::new(time.value()).ok();;
    }
}
