use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::state_space::StateSpace::StateSpace;
use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_RRF_LAM;

impl TraitStateTransitionFunction for STF_RRF_LAM {
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
        let nominal_interest_rate = states.nominalInterestRate.expect("nominalInterestRate should always be Some");
        let interest_calculation_base_amount = states.interestCalculationBaseAmount.expect("interestCalculationBaseAmount should always be Some");
        let notional_principal = states.notionalPrincipal.expect("notionalPrincipal should always be Some");

        let fee_rate = model.fee_rate.clone().expect("fee rate should always be Some");
        let next_reset_rate = model.nextResetRate.clone().expect("fee rate should always be Some");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date),
            time_adjuster.shift_sc(time),
        );

        states.accruedInterest = states.accruedInterest.map(|accrued_interest| {
            accrued_interest + nominal_interest_rate * interest_calculation_base_amount * time_from_last_event
        });

        states.feeAccrued = states.feeAccrued.map(|fee_accrued| {
            let fee_rate = fee_rate;
            fee_accrued + fee_rate * notional_principal * time_from_last_event
        });

        states.nominalInterestRate = Some(next_reset_rate);
        states.statusDate = Some(*time);
    }
}
