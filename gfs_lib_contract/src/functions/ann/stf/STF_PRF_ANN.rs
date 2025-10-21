use std::sync::Arc;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::terms::grp_notional_principal::NextPrincipalRedemptionPayment::NextPrincipalRedemptionPayment;
use gfs_lib_terms::traits::TraitOptionExt::TraitOptionExt;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use gfs_lib_types::traits::TraitConvert::IsoDateTimeConvertTo;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::util::RedemptionUtils::RedemptionUtils;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct STF_PRF_ANN;
impl TraitStateTransitionFunction for STF_PRF_ANN {
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
    )  {

        let status_date = states.status_date.clone().expect("status date should be some");
        let nominal_interest_rate = states.nominal_interest_rate.clone().expect("nominalInterestRate should be some");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.clone().expect("nominalInterestRate should be some");
        let notional_principal = states.notional_principal.clone().expect("notionalPrincipal should be some");

        let fee_rate_m = contract_terms.clone().fee_rate.clone().expect("feeRate should be some");
        let contract_role_m = contract_terms.clone().contract_role.clone().expect("contract role should be some");
        let day_counter = day_counter.clone().expect("sould have day counter");

        let time_from_last_event = day_counter.day_count_fraction(time_adjuster.shift_sc(&status_date.convert::<PhantomIsoDatetimeW>()),
                                                                  time_adjuster.shift_sc(time));

        states.accrued_interest.add_assign(time_from_last_event *
            nominal_interest_rate.value() * interest_calculation_base_amount.value());

        states.fee_accrued.add_assign(time_from_last_event * notional_principal.value() * fee_rate_m.value());

        states.status_date = StatusDate::new(time.value()).ok();
        let a = contract_role_m.clone().role_sign();
        let b = RedemptionUtils::redemptionAmount(&contract_terms, &states);
        states.next_principal_redemption_payment = NextPrincipalRedemptionPayment::new(
            contract_role_m.clone().role_sign() * RedemptionUtils::redemptionAmount(&contract_terms, &states)).ok(); // implementer redemptionm utile
        println!("ok");
    }
}
