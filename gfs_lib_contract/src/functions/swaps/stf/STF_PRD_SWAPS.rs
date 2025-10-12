use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use crate::attributes::ContractTerms::ContractTerms;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::attributes::ContractReference::ContractReference;
use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;

#[allow(non_camel_case_types)]
pub struct STF_PRD_SWAPS;

impl TraitStateTransitionFunction for STF_PRD_SWAPS {
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &mut StatesSpace,
        _contract_terms: &ContractTerms,
contract_structure: &Option<Vec<ContractReference>>,
        _risk_factor_model: &Option<impl TraitRiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        states.status_date = StatusDate::new(time.value()).ok();
    }
}
