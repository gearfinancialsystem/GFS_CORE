use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_MD_FUTUR;

impl TraitStateTransitionFunction for STF_MD_FUTUR {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        // Placeholder for risk factor model state retrieval
        let st = 1.0; // Placeholder value for demonstration
        let futures_price = model.futuresPrice.unwrap_or(0.0);
        let x = st - futures_price;

        if x == 0.0 {
            states.exerciseDate = None;
        } else {
            states.exerciseDate = Some(*time);
        }

        states.statusDate = Some(*time);
    }
}
