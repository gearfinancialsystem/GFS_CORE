use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_interest::interest_calculation_base::Nt::NT;
use lib_actus_terms::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use lib_actus_terms::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
use lib_actus_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;

use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;

use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;

#[allow(non_camel_case_types)]
pub struct STF_IED_LAM;

impl TraitStateTransitionFunction for STF_IED_LAM {
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
        let contract_role = contract_terms.contract_role.clone().expect("contractRole should always be Some");
        let notional_principal = contract_terms.notional_principal.clone().expect("notionalPrincipal should always be Some");
        let nominal_interest_rate = contract_terms.nominal_interest_rate.clone().expect("nominalInterestRate should always be Some");

        states.status_date = StatusDate::new(time.value()).ok();
        states.notional_principal = NotionalPrincipal::new(contract_role.role_sign() * notional_principal.value()).ok();
        states.nominal_interest_rate = Some(nominal_interest_rate);

        if let Some(interest_calculation_base) = &contract_terms.interest_calculation_base {
            if *interest_calculation_base == InterestCalculationBase::NT(NT) {
                states.interest_calculation_base_amount = InterestCalculationBaseAmount::new(states.notional_principal.clone().unwrap().value()).ok() ;
            } else {
                let interest_calculation_base_amount = contract_terms.interest_calculation_base_amount.clone().expect("interestCalculationBaseAmount should always be Some");
                states.interest_calculation_base_amount = InterestCalculationBaseAmount::new(contract_role.role_sign() * interest_calculation_base_amount.value()).ok();
            }
        }

        if let Some(accrued_interest) = contract_terms.accrued_interest.clone() {
            states.accrued_interest = AccruedInterest::new(contract_role.role_sign() * accrued_interest.value()).ok();
        } else if let Some(cycle_anchor_date_of_interest_payment) = contract_terms.cycle_anchor_date_of_interest_payment.clone() {
            if cycle_anchor_date_of_interest_payment.to_phantom_type() < *time {
                let time_from_last_event = day_counter.day_count_fraction(
                    time_adjuster.shift_sc(&cycle_anchor_date_of_interest_payment.to_phantom_type()),
                    time_adjuster.shift_sc(time)
                );

                states.accrued_interest = AccruedInterest::new({
                    states.notional_principal.clone().unwrap().value() *
                        {
                            if states.interest_calculation_base_amount.is_none() {
                                0.0
                            }
                            else {
                                states.interest_calculation_base_amount.clone().unwrap().value()
                            }
                        } * time_from_last_event
                }).ok();

            } else {
                states.accrued_interest = AccruedInterest::new(0.0).ok();
            }
        } else {
            states.accrued_interest = AccruedInterest::new(0.0).ok();
        }
    }
}
