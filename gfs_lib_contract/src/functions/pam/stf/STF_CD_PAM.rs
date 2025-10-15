use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use gfs_lib_terms::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use gfs_lib_terms::traits::TraitOptionExt::TraitOptionExt;

// use crate::attributes::ContractReference::ContractReference;
use crate::traits::_TraitRiskFactorModel::TraitRiskFactorModel;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use gfs_lib_types::traits::TraitConvert::IsoDateTimeConvertTo;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::traits::TraitExternalData::TraitExternalData;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct STF_CD_PAM;

impl TraitStateTransitionFunction for STF_CD_PAM {
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
        let status_date = states.status_date.as_ref().expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.as_ref().expect("nominalInterestRate should always be Some");
        let notional_principal = states.notional_principal.as_ref().expect("notionalPrincipal should always be Some");
        let fee_rate = contract_terms.fee_rate.as_ref().expect("fee rate should be Some");
        
        let time_from_last_event = day_counter.day_count_fraction(time_adjuster.shift_sc(
            &{
                let tmp: PhantomIsoDatetimeW = status_date.convert();
                tmp
            }),
          time_adjuster.shift_sc(time));
        
        states.accrued_interest.add_assign(nominal_interest_rate.value() * notional_principal.value() * time_from_last_event);
        states.fee_accrued.add_assign(fee_rate.value() * notional_principal.value() * time_from_last_event);
        states.contract_performance = Some(ContractPerformance::new("DF").expect("ok cp"));
        states.status_date = StatusDate::new(time.value()).ok();

    }
}
