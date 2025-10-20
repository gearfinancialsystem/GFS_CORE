use std::sync::Arc;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
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
pub struct STF_PR2_NAM;

impl TraitStateTransitionFunction for STF_PR2_NAM {
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
        let day_counter = day_counter.clone().expect("sould have day counter");
        let status_date = states.status_date.clone().expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.clone().expect("nominalInterestRate should always be Some");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.clone().expect("interestCalculationBaseAmount should always be Some");
        let notional_principal = states.notional_principal.clone().expect("notionalPrincipal should always be Some");
        let next_principal_redemption_payment = states.next_principal_redemption_payment.clone().expect("nextPrincipalRedemptionPayment should always be Some");
        //let contract_role = contract_terms.contract_role.clone().expect("contract role should always be Some");
        let _accrued_interest = states.accrued_interest.clone().expect("accruedInterest should always be Some");

        let fee_rate_m = contract_terms.fee_rate.clone().expect("feeRate model should be Some");


        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.convert::<PhantomIsoDatetimeW>()),
            time_adjuster.shift_sc(time)
        );

        states.accrued_interest.add_assign(nominal_interest_rate.value() * interest_calculation_base_amount.value() * time_from_last_event);
        states.fee_accrued.add_assign(fee_rate_m.value() * notional_principal.value() * time_from_last_event);


        let contract_role = contract_terms.contract_role.as_ref().expect("contractRole should always be Some");
        let role_sign = contract_role.role_sign();
        let redemption_amount = next_principal_redemption_payment.value() -
            (role_sign * states.accrued_interest.clone().unwrap().value());

        let redemption = redemption_amount -
            (0.0_f64.max( redemption_amount - notional_principal.value().abs()));

        states.notional_principal.sub_assign(role_sign * redemption);



        states.interest_calculation_base_amount = InterestCalculationBaseAmount::new(states.notional_principal.clone().unwrap().value()).ok();
        states.status_date = StatusDate::new(time.value()).ok();
    }
}
