use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
// use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_fees::FeeAccrued::FeeAccrued;
use lib_actus_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
use lib_actus_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;

use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;

use lib_actus_terms::traits::TraitOptionExt::TraitOptionExt;
use lib_actus_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::traits::_TraitRiskFactorModel::TraitRiskFactorModel;
use crate::traits::TraitExternalData::TraitExternalData;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct STF_IPCI2_LAM;

impl TraitStateTransitionFunction for STF_IPCI2_LAM {
    fn new() -> Self {
        Self {}
    }
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &mut StatesSpace,
        contract_terms: &ContractTerms,
        _contract_structure: &Option<RelatedContracts>,
        _risk_factor_external_data: &Option<Box<dyn TraitExternalData>>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let status_date = states.status_date.clone().expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.clone().expect("nominalInterestRate should always be Some");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.clone().expect("interestCalculationBaseAmount should always be Some");
        let notional_principal = states.notional_principal.clone().expect("notional_principal should always be Some");
        let accrued_interest = states.accrued_interest.clone().expect("accrued_interest should always be Some");
        let fee_accrued_m = contract_terms.fee_accrued.clone().expect("fee accrued should always be Some");
        let fee_rate_m = contract_terms.fee_rate.clone().expect("fee rate should always be Some");


        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.clone().to_phantom_type()),
            time_adjuster.shift_sc(time)
        );


        states.notional_principal.add_assign(accrued_interest.value() + (nominal_interest_rate.value() * interest_calculation_base_amount.value() * time_from_last_event));


        states.accrued_interest = AccruedInterest::new(0.0).ok();
    
        states.fee_accrued.add_assign(fee_rate_m.value() * notional_principal.value() * time_from_last_event);


        states.status_date = StatusDate::new(time.value()).ok();
        states.interest_calculation_base_amount = InterestCalculationBaseAmount::new(states.notional_principal.clone().unwrap().value()).ok();
    }
}
