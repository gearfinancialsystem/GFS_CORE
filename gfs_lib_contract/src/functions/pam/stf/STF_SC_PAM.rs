use std::sync::Arc;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::terms::grp_notional_principal::InterestScalingMultiplier::InterestScalingMultiplier;
use gfs_lib_terms::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use gfs_lib_terms::traits::TraitOptionExt::TraitOptionExt;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use gfs_lib_types::traits::TraitConvert::IsoDateTimeConvertTo;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::traits::TraitExternalData::TraitExternalData;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct STF_SC_PAM;

impl TraitStateTransitionFunction for STF_SC_PAM {
    fn new() -> Self {
        Self {}
    }
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &mut StatesSpace,
        contract_terms: &ContractTerms,
        _contract_structure: &Option<RelatedContracts>,
        risk_factor_external_data: &Option<Arc<dyn TraitExternalData>>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) { // ->StateSpace
        let day_counter = day_counter.clone().expect("sould have day counter");
        let status_date = states.status_date.as_ref().expect("status date should always be some");
        let nominal_interest_rate = states.nominal_interest_rate.as_ref().expect("nominalInterestRate should always be None");
        let notional_principal = states.notional_principal.as_ref().expect("notionalPrincipal should always be None");
        let fee_rate = contract_terms.fee_rate.as_ref().expect("fee rate should always be None");
        let scaling_effect = contract_terms.scaling_effect.as_ref().expect("scalingEffect should always be None");
        
        let time_from_last_event = day_counter.day_count_fraction(time_adjuster.shift_sc(
            &{
                let tmp: PhantomIsoDatetimeW = status_date.convert();
                tmp
            },
        ),
        time_adjuster.shift_sc(time));


        states.accrued_interest.add_assign(nominal_interest_rate.value() * notional_principal.value() * time_from_last_event);
        states.fee_accrued.add_assign(fee_rate.value() * notional_principal.value() * time_from_last_event);

        let cbv = if let Some(rfm) = risk_factor_external_data {
            rfm.state_at(
                contract_terms.market_object_code_of_scaling_index.clone().unwrap().value(),
                time
            )
        } else {
            None
        };

        if scaling_effect.to_string().contains("I") {
            states.interest_scaling_multiplier = InterestScalingMultiplier::new(cbv.unwrap() / contract_terms.scaling_index_at_contract_deal_date.unwrap().value()).ok();//
        }
        if scaling_effect.to_string().contains("N") {
            states.notional_scaling_multiplier = NotionalScalingMultiplier::new(cbv.unwrap() / contract_terms.scaling_index_at_contract_deal_date.unwrap().value()).ok();//
        }
        
        states.status_date = StatusDate::new(time.value()).ok();
        
    }
}
