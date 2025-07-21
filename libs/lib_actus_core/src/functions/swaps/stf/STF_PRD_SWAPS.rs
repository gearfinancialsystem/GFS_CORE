use crate::attributes::ContractTerms::ContractTerms;
use crate::external::RiskFactors::RiskFactors;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_PRD_SWAPS;

impl TraitStateTransitionFunction for STF_PRD_SWAPS {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        _model: &ContractTerms,
        _risk_factor_model: &RiskFactors,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        states.status_date = Some(StatusDate::from(*time));
    }
}
