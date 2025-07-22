use std::ops::Div;
use crate::attributes::ContractTerms::ContractModel;

use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_optionality::OptionType::OptionType;
use crate::terms::grp_optionality::option_type::C::C;
use crate::terms::grp_optionality::option_type::P::P;
use crate::terms::grp_settlement::ExerciseDate::ExerciseDate;
use crate::traits::TraitOptionExt::TraitOptionExt;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;

#[allow(non_camel_case_types)]
pub struct STF_MD_OPTNS;

impl TraitStateTransitionFunction for STF_MD_OPTNS {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        _risk_factor_model: &DataObserver,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        if states.exercise_date.is_none() {
            let mut x = 0.0;
            let st = 1.0; // Placeholder for risk_factor_model logic
            let option_type = model.option_type.as_ref().expect("optionType should always be Some");
            let option_strike1 = model.option_strike1.clone().itself_or(0.0);

            match option_type {
                OptionType::C(C) => {
                    x = (st - option_strike1.value()).max(0.0);
                },
                OptionType::P(P) => {
                    x = (option_strike1.value() - st).max(0.0);
                },
                _ => {
                    let option_strike2 = model.option_strike2.itself_or(0.0).value();
                    x = (st - option_strike1.value()).max(0.0) + (option_strike2 - st).max(0.0); //ERREUR ICI
                }
            }

            if x == 0.0 {
                states.exercise_date = None;
            } else {
                states.exercise_date = Some(ExerciseDate::from(*time));
            }
        }
        states.status_date = Some(StatusDate::from(*time));
    }
}
