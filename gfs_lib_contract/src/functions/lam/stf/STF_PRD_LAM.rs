use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
// use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use gfs_lib_terms::terms::grp_fees::FeeAccrued::FeeAccrued;
use gfs_lib_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;


use gfs_lib_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use gfs_lib_types::traits::TraitConvert::IsoDateTimeConvertTo;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::traits::_TraitRiskFactorModel::TraitRiskFactorModel;
use crate::traits::TraitExternalData::TraitExternalData;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct STF_PRD_LAM;

impl TraitStateTransitionFunction for STF_PRD_LAM {
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
        let notional_principal = states.notional_principal.clone().expect("notionalPrincipal should always be Some");

        let time_from_last_event = day_counter.day_count_fraction(
            {
                let tmp : PhantomIsoDatetimeW = status_date.convert();
                time_adjuster.shift_sc(&tmp)
            },
            time_adjuster.shift_sc(time)
        );

        states.accrued_interest = AccruedInterest::new({
            states.accrued_interest.clone().unwrap().value() + nominal_interest_rate.value() * interest_calculation_base_amount.value() * time_from_last_event
        }).ok();

        states.fee_accrued = FeeAccrued::new({
            let fee_rate = {
                if contract_terms.fee_rate.is_none() {
                    0.0
                }
                else { contract_terms.fee_rate.clone().unwrap().value() }
            };
            states.fee_accrued.clone().unwrap().value() + fee_rate * {
                if states.notional_principal.is_none() {
                    0.0
                } else {states.notional_principal.clone().unwrap().value()}
            } * time_from_last_event
        }).ok();

        states.status_date = StatusDate::new(time.value()).ok();
    }
}
