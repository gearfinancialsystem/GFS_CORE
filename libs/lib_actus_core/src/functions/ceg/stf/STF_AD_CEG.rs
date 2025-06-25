use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;
use crate::terms::grp_fees::FeeBasis::FeeBasis;
use crate::terms::grp_fees::fee_basis::A::A;
use crate::util::CycleUtils::CycleUtils;

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
        let status_date = states.statusDate.expect("statusDate should always be Some");
        let shifted_status_date = time_adjuster.shift_sc(&status_date);
        let shifted_time = time_adjuster.shift_sc(time);

        let fee_rate = model.feeRate.unwrap_or(0.0);

        if fee_rate == 0.0 {
            // No change to feeAccrued if feeRate is 0.0
        } else if let FeeBasis::A(A) = model.feeBasis.clone().unwrap() {
            if let Some(cycle_of_fee) = &model.cycleOfFee {
                let time_from_last_event = day_counter.day_count_fraction(shifted_status_date, shifted_time);

                let cycle_period = CycleUtils::parse_period(cycle_of_fee);
                let future_status_date = status_date + cycle_period.unwrap();
                let shifted_future_status_date = time_adjuster.shift_sc(&future_status_date);

                let time_full_fee_cycle = day_counter.day_count_fraction(shifted_status_date, shifted_future_status_date);

                let contract_role = model.contractRole.as_ref().expect("contractRole should always be Some");
                let role_sign = contract_role.role_sign();

                states.feeAccrued = states.feeAccrued.map(|mut fee_accrued| {
                    fee_accrued += role_sign * time_from_last_event / time_full_fee_cycle * fee_rate;
                    fee_accrued
                });
            }
        } else {
            let time_from_last_event = day_counter.day_count_fraction(shifted_status_date, shifted_time);
            let notional_principal = states.notionalPrincipal.unwrap_or(0.0);

            states.feeAccrued = states.feeAccrued.map(|mut fee_accrued| {
                fee_accrued += notional_principal * time_from_last_event * fee_rate;
                fee_accrued
            });
        }

        states.statusDate = Some(*time);
    }
}
