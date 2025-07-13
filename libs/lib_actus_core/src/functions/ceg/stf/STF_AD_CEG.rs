use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::terms::grp_fees::FeeBasis::FeeBasis;
use crate::terms::grp_fees::fee_basis::A::A;
use crate::terms::grp_fees::FeeRate::FeeRate;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::traits::TraitMarqueurIsoCycle::TraitMarqueurIsoCycle;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;


#[allow(non_camel_case_types)]
pub struct STF_AD_CEG;

impl TraitStateTransitionFunction for STF_AD_CEG {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let status_date = states.status_date.clone().expect("statusDate should always be Some");
        let shifted_status_date = time_adjuster.shift_sc(&status_date.value());
        let shifted_time = time_adjuster.shift_sc(time);

        //let fee_rate = model.fee_rate.clone().unwrap_or(0.0);
        let fee_rate = {
            if model.notional_principal.is_none() {
                FeeRate::new(0.0).ok()
            }
            else {
                model.fee_rate.clone()
            }
        }.unwrap();
        if fee_rate.value() == 0.0 {
            // No change to feeAccrued if feeRate is 0.0
        } else if let FeeBasis::A(A) = model.fee_basis.clone().unwrap() {
            if let Some(cycle_of_fee) = &model.cycle_of_fee {
                let time_from_last_event = day_counter.day_count_fraction(shifted_status_date, shifted_time);

                let cycle_period = cycle_of_fee.value().extract_period().unwrap();

                    //CycleUtils::parse_period(cycle_of_fee);
                let future_status_date = status_date + cycle_period;
                let shifted_future_status_date = time_adjuster.shift_sc(&future_status_date.value());

                let time_full_fee_cycle = day_counter.day_count_fraction(shifted_status_date, shifted_future_status_date);

                let contract_role = model.contract_role.as_ref().expect("contractRole should always be Some");
                let role_sign = contract_role.role_sign();

                states.fee_accrued = states.fee_accrued.clone().map(|mut fee_accrued| {
                    fee_accrued += role_sign * time_from_last_event / time_full_fee_cycle * fee_rate.value();
                    fee_accrued
                });
            }
        } else {
            let time_from_last_event = day_counter.day_count_fraction(shifted_status_date, shifted_time);
            let notional_principal = {
                if states.notional_principal.is_none() {
                    NotionalPrincipal::new(0.0).ok()
                }
                else {
                    states.notional_principal.clone()
                }
            }.unwrap();


            states.fee_accrued = states.fee_accrued.clone().map(|mut fee_accrued| {
                fee_accrued += notional_principal.value() * time_from_last_event * fee_rate.value();
                fee_accrued
            });
        }

        states.status_date = Some(StatusDate::from(*time));
    }
}
