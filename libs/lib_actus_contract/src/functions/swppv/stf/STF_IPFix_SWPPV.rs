use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;

use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::external::RiskFactorModel::RiskFactorModel;
use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use crate::attributes::ContractReference::ContractReference;

#[allow(non_camel_case_types)]
pub struct STF_IPFix_SWPPV;

impl TraitStateTransitionFunction for STF_IPFix_SWPPV {
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &mut StatesSpace,
        _contract_terms: &ContractTerms,
contract_structure: &Option<Vec<ContractReference>>,
        _risk_factor_model: &Option<impl TraitRiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let status_date = states.status_date.clone().expect("statusDate should always be Some");

        states.last_interest_period = Some(day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.value()),
            time_adjuster.shift_sc(time)
        ));

        states.accrued_interest = AccruedInterest::new(0.0).ok();
        states.status_date = StatusDate::new(time.value()).ok();
    }
}
