use lib_actus_terms::phantom_terms::PhantomF64::PhantomF64W;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;

use lib_actus_types::types::IsoDatetime::IsoDatetime;
// use crate::attributes::ContractReference::ContractReference;
use crate::traits::_TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use lib_actus_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::traits::TraitExternalData::TraitExternalData;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct STF_IED_PAM;

impl TraitStateTransitionFunction for STF_IED_PAM {
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
    )  {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let contract_role = contract_terms.contract_role.as_ref().expect("contract role should be Some");
        let notional_principal = contract_terms.notional_principal.as_ref().expect("notionalPrincipal should always be Some");
        let nominal_interest_rate = contract_terms.nominal_interest_rate.clone().expect("nominalInterestRate should be Some");
        let notional_principal_s = states.notional_principal.clone().expect("notionalPrincipal should always be Some");
        let nominal_interest_rate_s = states.nominal_interest_rate.clone().expect("nominalInterestRate should be Some");
        
        
        states.notional_principal = NotionalPrincipal::new(contract_role.role_sign() * notional_principal.value()).ok();
        states.nominal_interest_rate = Some(nominal_interest_rate);
        states.status_date = StatusDate::new(time.value()).ok();

        if let (Some(cycle_anchor_date), Some(initial_exchange_date)) = (
            contract_terms.cycle_anchor_date_of_interest_payment.as_ref(),
            contract_terms.initial_exchange_date.as_ref(),
        ) {
            if cycle_anchor_date.value() < initial_exchange_date.value() {
                states.accrued_interest = states.accrued_interest.clone().map(|mut accrued_interest| {
                    accrued_interest += notional_principal_s * nominal_interest_rate_s *
                        PhantomF64W::new(day_counter.day_count_fraction(
                            time_adjuster.shift_sc(&cycle_anchor_date.to_phantom_type()),
                            time_adjuster.shift_sc(time)
                        )).expect("Should be Some");
                    accrued_interest
                });
            }
        }
        
        
    }
}
