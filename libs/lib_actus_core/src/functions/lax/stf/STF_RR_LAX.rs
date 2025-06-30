use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_RR_LAX {
    scheduled_rate: f64,
}

impl STF_RR_LAX {
    pub fn new(rate: f64) -> Self {
        STF_RR_LAX { scheduled_rate: rate }
    }
}

impl TraitStateTransitionFunction for STF_RR_LAX {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        // Compute new rate
        let rate = (1.0 * model.rateMultiplier.unwrap_or(1.0)) // Placeholder for risk_factor_model logic
            + model.rateSpread.unwrap_or(0.0)
            + self.scheduled_rate
            - states.nominalInterestRate.unwrap_or(0.0);

        let delta_rate = rate.max(model.periodFloor.unwrap_or(f64::MIN)).min(model.periodCap.unwrap_or(f64::MAX));

        let new_rate = (states.nominalInterestRate.unwrap_or(0.0) + delta_rate)
            .max(model.lifeFloor.unwrap_or(f64::MIN))
            .min(model.lifeCap.unwrap_or(f64::MAX));

        // Update state space
        let status_date = states.statusDate.expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominalInterestRate.unwrap_or(0.0);
        let interest_calculation_base_amount = states.interestCalculationBaseAmount.unwrap_or(0.0);

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date),
            time_adjuster.shift_sc(time)
        );

        states.accruedInterest = states.accruedInterest.map(|accrued_interest| {
            accrued_interest + nominal_interest_rate * interest_calculation_base_amount * time_from_last_event
        });

        states.feeAccrued = states.feeAccrued.map(|fee_accrued| {
            let fee_rate = model.feeRate.unwrap_or(0.0);
            fee_accrued + fee_rate * states.notionalPrincipal.unwrap_or(0.0) * time_from_last_event
        });

        states.nominalInterestRate = Some(new_rate);
        states.statusDate = Some(*time);
    }
}
