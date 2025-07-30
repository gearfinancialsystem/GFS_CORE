use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_optionality::OptionType::OptionType;
use crate::terms::grp_optionality::option_type::C::C;
use crate::terms::grp_optionality::option_type::P::P;
use crate::terms::grp_settlement::ExerciseAmount::ExerciseAmount;

use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::traits::TraitOptionExt::TraitOptionExt;
use crate::attributes::ContractReference::ContractReference;
#[allow(non_camel_case_types)]
pub struct STF_XD_OPTNS;

impl TraitStateTransitionFunction for STF_XD_OPTNS {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        contract_terms: &ContractTerms,
        contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<RiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
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
        let option_strike1 = contract_terms.option_strike1.clone().itself_or(0.0).value();

        states.exercise_amount = match option_type {
            OptionType::C(C) => ExerciseAmount::new((cbv.unwrap() - option_strike1).max(0.0)).ok(),
            OptionType::P(P) => ExerciseAmount::new((option_strike1 - cbv.unwrap()).max(0.0)).ok(),
            _ => {
                let option_strike2 = contract_terms.option_strike2.clone().itself_or(0.0);
                ExerciseAmount::new((cbv.unwrap() - option_strike1).max(0.0) + (option_strike2.value() - cbv.unwrap()).max(0.0)).ok()
            }
        };

        states.status_date = Some(StatusDate::from(*time));
    }
}
