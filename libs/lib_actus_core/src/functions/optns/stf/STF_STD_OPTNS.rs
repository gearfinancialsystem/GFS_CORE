use crate::state_space::StateSpace::StateSpace;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::attributes::ContractTerms::ContractModel;

use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_settlement::ExerciseAmount::ExerciseAmount;
use crate::util_tests::essai_data_observer::DataObserver;

#[allow(non_camel_case_types)]
pub struct STF_STD_OPTNS;

impl TraitStateTransitionFunction for STF_STD_OPTNS {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        _model: &ContractModel,
        _risk_factor_model: &DataObserver,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        states.exercise_amount = ExerciseAmount::new(0.0).ok();
        states.status_date = Some(StatusDate::from(*time));
    }
}
