use crate::attributes::ContractModel::ContractModel;
use crate::terms::grp_contract_identification::contract_types::Ceg::CEG;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
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
use crate::traits::TraitMarqueurIsoCycle::TraitMarqueurIsoCycle;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::traits::TraitOptionExt::TraitOptionExt;

#[allow(non_camel_case_types)]
pub struct STF_XD_CEG;

impl TraitStateTransitionFunction for STF_XD_CEG {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let day_counter = day_counter.clone().expect("sould have day counter");
        // Set notionalPrincipal if it is not already set
        if model.notional_principal.is_none() {
            states.notional_principal =NotionalPrincipal::new(CEG::calculate_notional_principal(
                states,
                model,
                risk_factor_model,
                time,
            )).ok();
        }

        states.exercise_amount = ExerciseAmount::new(states.notional_principal.clone().unwrap().value()).ok();
        states.exercise_date = ExerciseDate::new(ExerciseDate::from(*time).value()).ok();

        let status_date = states.status_date.clone().expect("statusDate should always be Some");
        let shifted_status_date = time_adjuster.shift_sc(&status_date.value());
        let shifted_time = time_adjuster.shift_sc(time);

        let fee_rate = model.fee_rate.itself_or(0.0);

        if fee_rate.value() == 0.0 {
            // No change to feeAccrued if feeRate is 0.0
        } else if let Some(FeeBasis::A(A)) = model.fee_basis {
            if let Some(cycle_of_fee) = &model.cycle_of_fee {
                let time_from_last_event = day_counter.day_count_fraction(shifted_status_date, shifted_time);

                let cycle_period = cycle_of_fee.value().extract_period().unwrap();
                let future_status_date = status_date + cycle_period;
                let shifted_future_status_date = time_adjuster.shift_sc(&future_status_date.value());

                let time_full_fee_cycle = day_counter.day_count_fraction(shifted_status_date, shifted_future_status_date);

                let contract_role = model.contract_role.as_ref().expect("contractRole should always be Some");
                let role_sign = contract_role.role_sign();
                
                states.fee_accrued.add_assign(role_sign * time_from_last_event / time_full_fee_cycle * fee_rate.value());
            }
        } else {
            let time_from_last_event = day_counter.day_count_fraction(shifted_status_date, shifted_time);
            let notional_principal = states.notional_principal.clone().itself_or(0.0);
            
            states.fee_accrued.add_assign(notional_principal.value() * time_from_last_event * fee_rate.value());
        }

        states.status_date = Some(StatusDate::from(*time));
    }
}
