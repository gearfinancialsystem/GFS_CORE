use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
//// use crate::attributes::ContractReference::ContractReference;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;

use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_fees::FeeAccrued::FeeAccrued;

use lib_actus_types::types::IsoDatetime::IsoDatetime;

use lib_actus_terms::traits::TraitOptionExt::TraitOptionExt;
use lib_actus_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::traits::_TraitRiskFactorModel::TraitRiskFactorModel;
use crate::traits::TraitExternalData::TraitExternalData;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct STF_FP_LAM;

impl TraitStateTransitionFunction for STF_FP_LAM {
    fn new() -> Self {
        Self {}
    }
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &mut StatesSpace,
        _contract_terms: &ContractTerms,
        contract_structure: &Option<RelatedContracts>,
        _risk_factor_external_data: &Option<Box<dyn TraitExternalData>>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    )  {
        let day_counter = day_counter.clone().expect("sould have day counter");
        // Create a mutable copy of the states to update

        // Create a mutable copy of the states to update
        let status_date = states.status_date.clone().expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.clone().expect("nominalInterestRate should always be Some");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.clone().expect("interestCalculationBaseAmount should always be Some");


        // Update state space
        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.clone().to_phantom_type()),
            time_adjuster.shift_sc(time),
        );

        states.accrued_interest.add_assign(nominal_interest_rate.value() * interest_calculation_base_amount.value() * time_from_last_event);

        states.fee_accrued = FeeAccrued::new(0.0).ok();
        states.status_date = StatusDate::new(time.value()).ok();


    }
}
