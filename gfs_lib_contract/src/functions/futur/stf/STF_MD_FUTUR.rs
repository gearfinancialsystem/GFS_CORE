use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_settlement::ExerciseDate::ExerciseDate;


use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::traits::TraitOptionExt::TraitOptionExt;

#[allow(non_camel_case_types)]
pub struct STF_MD_FUTUR;

impl TraitStateTransitionFunction for STF_MD_FUTUR {
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


        let mut cbv = None;
        if let Some(rfm) = risk_factor_model {
            cbv = rfm.state_at(
                contract_structure.clone().unwrap().get(0).unwrap().object.as_cm().unwrap().market_object_code.clone().unwrap().value(),
                time,
                states,
                contract_terms,
                true
            );
        } else {
            cbv = None
        }



        let futures_price = contract_terms.futures_price.clone().itself_or(0.0);
        let x = cbv.unwrap() - futures_price.value();

        if x == 0.0 {
            states.exercise_date = None;
        } else {
            states.exercise_date = Some(ExerciseDate::from(*time));
        }

        states.status_date = StatusDate::new(time.value()).ok();;
    }
}
