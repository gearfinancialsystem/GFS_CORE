use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;


#[allow(non_camel_case_types)]
pub struct STF_RR_PAM;

impl TraitStateTransitionFunction for STF_RR_PAM {
    fn eval(
        &self,
        time: &IsoDatetime, 
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let rate_multiplier = model.rateMultiplier.expect("rate_multiplier should be some");
        let rate_spread = model.rateSpread.expect("rate_spread should be some");
        let status_date = states.status_date.expect("status date should be some");
        let nominal_interest_rate = states.nominal_interest_rate.expect("nominalInterestRate should be some");
        let notional_principal = states.notional_principal.expect("notionalPrincipal should be some");
        let period_floor = model.periodFloor.expect("period floor should be some");
        let period_cap = model.periodCap.expect("period cap should be some");
        let life_floor = model.lifeFloor.expect("lifeFloor should be some");
        let life_cap = model.lifeCap.expect("lifeCap should be some");
        
        let mut rate = 1.0 * rate_multiplier + rate_spread;
        let mut delta_rate = rate - nominal_interest_rate;

        delta_rate = delta_rate.max(period_floor).min(period_cap);
        rate = nominal_interest_rate + delta_rate;
        rate = rate.max(life_floor).min(life_cap);

        let time_from_last_event = day_counter.day_count_fraction(time_adjuster.shift_sc(&status_date),
                                                                  time_adjuster.shift_sc(time));

        states.accrued_interest = states.accrued_interest.map(|mut accrued_interest| {
            accrued_interest += nominal_interest_rate * notional_principal * time_from_last_event;
            accrued_interest
        });
        
        states.nominal_interest_rate = Some(rate);

        states.status_date = Some(*time);


    }
}
