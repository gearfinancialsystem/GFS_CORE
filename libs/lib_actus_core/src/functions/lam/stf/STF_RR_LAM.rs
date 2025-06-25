use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::state_space::StateSpace::StateSpace;
use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::types::isoDatetime::IsoDatetime;

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
        // Create a mutable copy of the states to update


        // Calculate time from last event
        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&states.statusDate),
            time_adjuster.shift_sc(time),
        );

        // Calculate rate adjustments
        let rate = (risk_factor_model.state_at(
            model.get_as("marketObjectCodeOfRateReset"),
            time,
            &states,
            model,
            true,
        ) * model.get_as::<f64>("rateMultiplier"))
            + model.get_as::<f64>("rateSpread")
            - states.nominalInterestRate;

        let delta_rate = rate.max(model.get_as::<f64>("periodFloor")).min(model.get_as::<f64>("periodCap"));

        let adjusted_rate = (states.nominalInterestRate + delta_rate)
            .max(model.get_as::<f64>("lifeFloor"))
            .min(model.get_as::<f64>("lifeCap"));

        // Update state space
        states.accruedInterest += states.nominalInterestRate
            * states.interestCalculationBaseAmount
            * time_from_last_event;

        states.feeAccrued += model.get_as::<f64>("feeRate")
            * states.notionalPrincipal
            * time_from_last_event;

        states.nominalInterestRate = adjusted_rate;
        states.statusDate = Some(*time);


    }
}
