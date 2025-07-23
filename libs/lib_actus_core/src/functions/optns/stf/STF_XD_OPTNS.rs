use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_optionality::OptionType::OptionType;
use lib_actus_terms::terms::grp_optionality::option_type::C::C;
use lib_actus_terms::terms::grp_optionality::option_type::P::P;
use lib_actus_terms::terms::grp_settlement::ExerciseAmount::ExerciseAmount;

use lib_actus_events::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_terms::traits::TraitOptionExt::TraitOptionExt;

#[allow(non_camel_case_types)]
pub struct STF_XD_OPTNS;

impl TraitStateTransitionFunction for STF_XD_OPTNS {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        model: &ContractTerms,
        _risk_factor_model: Option<&dyn TraitRiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        let st = 1.0; // Placeholder for risk_factor_model logic
        let option_type = model.option_type.as_ref().expect("optionType should always be Some");
        let option_strike1 = model.option_strike1.clone().itself_or(0.0).value();

        states.exercise_amount = match option_type {
            OptionType::C(C) => ExerciseAmount::new((st - option_strike1).max(0.0)).ok(),
            OptionType::P(P) => ExerciseAmount::new((option_strike1 - st).max(0.0)).ok(),
            _ => {
                let option_strike2 = model.option_strike2.clone().itself_or(0.0);
                ExerciseAmount::new((st - option_strike1).max(0.0) + (option_strike2.value() - st).max(0.0)).ok()
            }
        };

        states.status_date = Some(StatusDate::from(*time));
    }
}
