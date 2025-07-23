use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_settlement::ExerciseDate::ExerciseDate;


use lib_actus_events::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_terms::traits::TraitOptionExt::TraitOptionExt;

#[allow(non_camel_case_types)]
pub struct STF_MD_FUTUR;

impl TraitStateTransitionFunction for STF_MD_FUTUR {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        model: &ContractTerms,
        risk_factor_model: Option<&dyn TraitRiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        // Placeholder for risk factor model state retrieval
        let st = risk_factor_model.state_at(
            model.contract_structure.clone().unwrap().0.get(0).unwrap().object.as_cm().unwrap().market_object_code.clone().unwrap().value(),
            time, states, model, true);
        let futures_price = model.futures_price.clone().itself_or(0.0);
        let x = st - futures_price.value();

        if x == 0.0 {
            states.exercise_date = None;
        } else {
            states.exercise_date = Some(ExerciseDate::from(*time));
        }

        states.status_date = Some(StatusDate::from(*time));;
    }
}
