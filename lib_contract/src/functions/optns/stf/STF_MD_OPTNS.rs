use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use std::ops::Div;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::ContractReference::ContractReference;
use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_optionality::OptionType::OptionType;
use crate::terms::grp_optionality::option_type::C::C;
use crate::terms::grp_optionality::option_type::P::P;
use crate::terms::grp_settlement::ExerciseDate::ExerciseDate;

use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::traits::TraitOptionExt::TraitOptionExt;

#[allow(non_camel_case_types)]
pub struct STF_MD_OPTNS;

impl TraitStateTransitionFunction for STF_MD_OPTNS {
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &mut StatesSpace,
        contract_terms: &ContractTerms,
        contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<impl TraitRiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        if states.exercise_date.is_none() {
            let mut x = 0.0;
            let mut cbv = None;
            if let Some(rfm) = risk_factor_model {
                cbv = rfm.state_at(
                    contract_structure.clone().unwrap().get(0).unwrap().object.as_cm().unwrap(),
                    time,
                    states,
                    contract_terms,
                    true
                );
            } else {
                cbv = None
            }
            let option_type = contract_terms.option_type.as_ref().expect("optionType should always be Some");
            let option_strike1 = contract_terms.option_strike1.clone().itself_or(0.0);

            match option_type {
                OptionType::C(C) => {
                    x = (cbv.unwrap() - option_strike1.value()).max(0.0);
                },
                OptionType::P(P) => {
                    x = (option_strike1.value() - cbv.unwrap()).max(0.0);
                },
                _ => {
                    let option_strike2 = contract_terms.option_strike2.itself_or(0.0).value();
                    x = (cbv.unwrap() - option_strike1.value()).max(0.0) + (option_strike2 - cbv.unwrap()).max(0.0); //ERREUR ICI
                }
            }

            if x == 0.0 {
                states.exercise_date = None;
            } else {
                states.exercise_date = Some(ExerciseDate::from(*time));
            }
        }
        states.status_date = StatusDate::new(time.value()).ok();
    }
}
