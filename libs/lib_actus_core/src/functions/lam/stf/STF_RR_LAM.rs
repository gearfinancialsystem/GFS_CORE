use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_RR_LAM;

impl TraitStateTransitionFunction for STF_RR_LAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let status_date = states.status_date.expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.expect("nominalInterestRate should always be Some");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.expect("interestCalculationBaseAmount should always be Some");
        let notional_principal = states.notional_principal.expect("notionalPrincipal should always be Some");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date),
            time_adjuster.shift_sc(time)
        );

        //let market_object_code_of_rate_reset = model.marketObjectCodeOfRateReset.as_ref().expect("marketObjectCodeOfRateReset should always be Some");
        let rate_multiplier = model.rateMultiplier.expect("rateMultiplier should always be Some");
        let rate_spread = model.rateSpread.expect("rateSpread should always be Some");
        let period_floor = model.periodFloor.expect("periodFloor should always be Some");
        let period_cap = model.periodCap.expect("periodCap should always be Some");
        let life_floor = model.lifeFloor.expect("lifeFloor should always be Some");
        let life_cap = model.lifeCap.expect("lifeCap should always be Some");
        // risk_factor_model.state_at(market_object_code_of_rate_reset, time, states, model, true) 
        let rate = ( 1.0 * rate_multiplier)
            + rate_spread - nominal_interest_rate;

        let delta_rate = rate.max(period_floor).min(period_cap);
        let new_rate = (nominal_interest_rate + delta_rate).max(life_floor).min(life_cap);

        states.accrued_interest = states.accrued_interest.map(|accrued_interest| {
            accrued_interest + nominal_interest_rate * interest_calculation_base_amount * time_from_last_event
        });

        states.fee_accrued = states.fee_accrued.map(|fee_accrued| {
            let fee_rate = model.fee_rate.unwrap_or(0.0);
            fee_accrued + fee_rate * notional_principal * time_from_last_event
        });

        states.nominal_interest_rate = Some(new_rate);
        states.status_date = Some(*time);
    }
}
