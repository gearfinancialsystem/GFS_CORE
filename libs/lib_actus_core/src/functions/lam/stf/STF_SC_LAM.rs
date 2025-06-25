use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::state_space::StateSpace::StateSpace;
use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_SC_LAM;

impl TraitStateTransitionFunction for STF_SC_LAM {
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


        // Update state space
        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&states.statusDate),
            time_adjuster.shift_sc(time),
        );

        states.accruedInterest += states.nominalInterestRate
            * states.interestCalculationBaseAmount
            * time_from_last_event;

        states.feeAccrued += model.get_as::<f64>("feeRate")
            * states.notionalPrincipal
            * time_from_last_event;

        let scaling_multiplier = risk_factor_model.state_at(
            model.get_as("marketObjectCodeOfScalingIndex"),
            time,
            &states,
            model,
            true,
        ) / model.get_as::<f64>("scalingIndexAtContractDealDate");

        let scaling_effect = model.get_as("scalingEffect").to_string();
        if scaling_effect.contains("I") {
            states.interest_scaling_multiplier = scaling_multiplier;
        }
        if scaling_effect.contains("N") {
            states.notional_scaling_multiplier = scaling_multiplier;
        }

        states.statusDate = Some(*time);


    }
}
