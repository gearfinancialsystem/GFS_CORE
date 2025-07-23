use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_events::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_settlement::ExerciseAmount::ExerciseAmount;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;

#[allow(non_camel_case_types)]
pub struct STF_STD_OPTNS;

impl TraitStateTransitionFunction for STF_STD_OPTNS {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        _model: &ContractTerms,
        _risk_factor_model: Option<&dyn TraitRiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        states.exercise_amount = ExerciseAmount::new(0.0).ok();
        states.status_date = Some(StatusDate::from(*time));
    }
}
