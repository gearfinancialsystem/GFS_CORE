use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_XD_FUTUR;

impl TraitStateTransitionFunction for STF_XD_FUTUR {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        // Placeholder for risk factor model state retrieval
        let st = RiskFactorModel.state_at(
            &model.contract_structure.clone().unwrap().get(0).unwrap().object.as_cm().unwrap().marketObjectCode.clone().unwrap(),
            time, states, model, true).expect("correct risk factor model");
        let futures_price = model.futures_price.unwrap_or(0.0);
        
        
        states.exercise_amount = Some(st - futures_price);
        states.status_date = Some(StatusDate::from(*time));
    }
}
