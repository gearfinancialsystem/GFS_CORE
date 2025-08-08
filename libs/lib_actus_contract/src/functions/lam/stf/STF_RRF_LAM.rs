use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use crate::attributes::ContractReference::ContractReference;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;

use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_fees::FeeAccrued::FeeAccrued;
use lib_actus_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use lib_actus_terms::terms::grp_interest::NominalInterestRate::NominalInterestRate;

use lib_actus_types::types::IsoDatetime::IsoDatetime;

use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;

#[allow(non_camel_case_types)]
pub struct STF_RRF_LAM;

impl TraitStateTransitionFunction for STF_RRF_LAM {
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &mut StatesSpace,
        contract_terms: &ContractTerms,
        _contract_structure: &Option<Vec<ContractReference>>,
        _risk_factor_model: &Option<impl TraitRiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let status_date = states.status_date.clone().expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.clone().expect("nominalInterestRate should always be Some");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.clone().expect("interestCalculationBaseAmount should always be Some");
        let notional_principal = states.notional_principal.clone().expect("notionalPrincipal should always be Some");

        let fee_rate_m = contract_terms.fee_rate.clone().expect("fee rate should always be Some");
        let next_reset_rate_m = contract_terms.next_reset_rate.clone().expect("fee rate should always be Some");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.clone().to_phantom_type()),
            time_adjuster.shift_sc(time),
        );


        states.accrued_interest = AccruedInterest::new({
            states.accrued_interest.clone().unwrap().value() + nominal_interest_rate.value() * interest_calculation_base_amount.value() * time_from_last_event
        }).ok();

        states.fee_accrued = FeeAccrued::new({

            states.fee_accrued.clone().unwrap().value() + fee_rate_m.value() * notional_principal.value() * time_from_last_event
        }).ok();

        states.nominal_interest_rate = NominalInterestRate::new(next_reset_rate_m.value()).ok();
        states.status_date = StatusDate::new(time.value()).ok();
    }
}
