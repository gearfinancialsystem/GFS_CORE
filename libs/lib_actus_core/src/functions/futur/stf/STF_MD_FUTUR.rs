use crate::attributes::ContractTerms::ContractModel;

use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_settlement::ExerciseDate::ExerciseDate;
use crate::traits::TraitOptionExt::TraitOptionExt;
use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;

#[allow(non_camel_case_types)]
pub struct STF_MD_FUTUR;

impl TraitStateTransitionFunction for STF_MD_FUTUR {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &DataObserver,
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
