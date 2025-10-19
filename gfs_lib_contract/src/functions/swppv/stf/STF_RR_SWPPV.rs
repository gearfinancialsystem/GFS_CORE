use std::sync::Arc;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use gfs_lib_terms::terms::grp_interest::AccruedInterest2::AccruedInterest2;
use gfs_lib_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use gfs_lib_terms::terms::grp_settlement::DeliverySettlement::DeliverySettlement;
use gfs_lib_terms::traits::TraitOptionExt::TraitOptionExt;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use crate::attributes::ContractTerms::ContractTerms;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use gfs_lib_types::traits::TraitConvert::IsoDateTimeConvertTo;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::traits::TraitExternalData::TraitExternalData;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct STF_RR_SWPPV;

impl TraitStateTransitionFunction for STF_RR_SWPPV {
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
    ) {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let status_date = states.status_date.clone().expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.clone().expect("nominalInterestRate should always be Some");
        let notional_principal = states.notional_principal.clone().expect("notionalPrincipal should always be Some");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.convert::<PhantomIsoDatetimeW>()),
            time_adjuster.shift_sc(time)
        );

        let model_nominal_interest_rate = contract_terms.nominal_interest_rate.clone().itself_or(0.0);
        let delivery_settlement = contract_terms.delivery_settlement.clone().expect("deliverySettlement should always be Some");

        let interest_rate = match delivery_settlement {
            DeliverySettlement::D(D) => model_nominal_interest_rate,
            _ => NominalInterestRate::new(model_nominal_interest_rate.value() - nominal_interest_rate.value()).expect(""),
        };

        states.accrued_interest = states.accrued_interest.clone().map(|mut accrued_interest| {
            accrued_interest +=
                AccruedInterest::new(interest_rate.value() * notional_principal.value() * time_from_last_event).expect("ok");
            accrued_interest
        });

        states.accrued_interest2 = states.accrued_interest2.clone().map(|mut accrued_interest2| {
            accrued_interest2 +=
                AccruedInterest2::new((-1.0) * nominal_interest_rate.value() * notional_principal.value() * time_from_last_event).expect("dsf");
            accrued_interest2
        });

        // Placeholder for risk factor calculation
        //let market_object_code_of_rate_reset = contract_terms.marketObjectCodeOfRateReset.as_ref().expect("marketObjectCodeOfRateReset should always be Some");
        let rate_multiplier = contract_terms.rate_multiplier.clone().itself_or(1.0);
        let rate_spread = contract_terms.rate_spread.clone().itself_or(0.0);

        // Simplified calculation as a placeholder
        let mut cbv = None;
        if let Some(rfm) = risk_factor_external_data {
            cbv = rfm.state_at(
                contract_terms.market_object_code_of_rate_reset.clone().unwrap().value(),
                time,
            );
        } else {
            cbv = None
        }

        states.nominal_interest_rate = NominalInterestRate::new(cbv.unwrap() * rate_multiplier.value() + rate_spread.value()).ok();

        states.status_date = StatusDate::new(time.value()).ok();;
    }
}
