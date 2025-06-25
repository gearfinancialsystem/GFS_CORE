use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::state_space::StateSpace::StateSpace;
use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::types::isoDatetime::IsoDatetime;
use crate::types::InterestCalculationBase::InterestCalculationBase;
use crate::utils::CommonUtils;

#[allow(non_camel_case_types)]
pub struct STF_PP_LAM;

impl TraitStateTransitionFunction for STF_PP_LAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    )  {
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

        let prepayment_factor = risk_factor_model.state_at(
            model.get_as("objectCodeOfPrepaymentModel"),
            time,
            &states,
            model,
            false
        );

        states.notionalPrincipal -= prepayment_factor * states.notionalPrincipal;

        if !CommonUtils::is_null(model.get_as("interestCalculationBase"))
            && model.get_as("interestCalculationBase") == InterestCalculationBase::NTL {
            states.interestCalculationBaseAmount = states.notionalPrincipal;
        }

        states.statusDate = Some(*time);


    }
}
