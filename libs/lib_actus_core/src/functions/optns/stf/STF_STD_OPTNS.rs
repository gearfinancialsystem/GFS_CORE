use crate::state_space::StateSpace::StateSpace;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_settlement::ExerciseAmount::ExerciseAmount;

#[allow(non_camel_case_types)]
pub struct STF_STD_OPTNS;

impl TraitStateTransitionFunction for STF_STD_OPTNS {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        _model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        states.exercise_amount = ExerciseAmount::new(0.0).ok();
        states.status_date = Some(StatusDate::from(*time));
    }
}
