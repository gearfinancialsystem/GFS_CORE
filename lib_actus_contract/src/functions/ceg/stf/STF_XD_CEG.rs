use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ContractTerms::ContractTerms;
use crate::contracts::Ceg::CEG;
use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::terms::grp_fees::FeeBasis::FeeBasis;
use crate::terms::grp_fees::fee_basis::A::A;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::terms::grp_settlement::ExerciseAmount::ExerciseAmount;
use crate::terms::grp_settlement::ExerciseDate::ExerciseDate;

use crate::external::RiskFactorModel::RiskFactorModel;
use crate::traits::TraitOptionExt::TraitOptionExt;
use crate::traits::TraitMarkerIsoCycle::TraitMarkerIsoCycle;
use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_XD_CEG;

impl TraitStateTransitionFunction for STF_XD_CEG {
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &mut StatesSpace,
        contract_terms: &ContractTerms,
        contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<impl TraitRiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let day_counter = day_counter.clone().expect("sould have day counter");
        // Set notionalPrincipal if it is not already set
        if contract_terms.notional_principal.is_none() {
            states.notional_principal = NotionalPrincipal::new(
                CEG::calculate_notional_principal(
                    contract_terms,
                    &contract_structure.clone().expect("should be one"),
                    &risk_factor_model.clone().expect("should have one"),
                    time)
            ).ok();
        }

        states.exercise_amount = ExerciseAmount::new(states.notional_principal.clone().unwrap().value()).ok();
        states.exercise_date = ExerciseDate::new(ExerciseDate::from(*time).value()).ok();

        let status_date = states.status_date.clone().expect("statusDate should always be Some");
        let shifted_status_date = time_adjuster.shift_sc(&status_date.value());
        let shifted_time = time_adjuster.shift_sc(time);

        let fee_rate = contract_terms.fee_rate.itself_or(0.0);

        if fee_rate.value() == 0.0 {
            // No change to feeAccrued if feeRate is 0.0
        } else if let Some(FeeBasis::A(A)) = contract_terms.fee_basis {
            if let Some(cycle_of_fee) = &contract_terms.cycle_of_fee {
                let time_from_last_event = day_counter.day_count_fraction(shifted_status_date, shifted_time);

                let cycle_period = cycle_of_fee.value().extract_period().unwrap();
                let future_status_date = status_date.value() + cycle_period;
                let shifted_future_status_date = time_adjuster.shift_sc(&future_status_date.value());

                let time_full_fee_cycle = day_counter.day_count_fraction(shifted_status_date, shifted_future_status_date);

                let contract_role = contract_terms.contract_role.as_ref().expect("contractRole should always be Some");
                let role_sign = contract_role.role_sign();
                
                states.fee_accrued.add_assign(role_sign * time_from_last_event / time_full_fee_cycle * fee_rate.value());
            }
        } else {
            let time_from_last_event = day_counter.day_count_fraction(shifted_status_date, shifted_time);
            let notional_principal = states.notional_principal.clone().itself_or(0.0);
            
            states.fee_accrued.add_assign(notional_principal.value() * time_from_last_event * fee_rate.value());
        }

        states.status_date = StatusDate::new(time.value()).ok();
    }
}
