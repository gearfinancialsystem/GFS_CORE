use crate::attributes::ContractTerms::ContractTerms;

use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::external::RiskFactorModels::RiskFactors;

#[allow(non_camel_case_types)]
pub struct STF_CD_FXOUT;

impl TraitStateTransitionFunction for STF_CD_FXOUT {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        _model: &ContractTerms,
        _risk_factor_model: &RiskFactors,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        // Update state space
        states.contract_performance = Some(ContractPerformance::new("DF").expect("ok cp"));
        states.status_date = Some(StatusDate::from(*time));
    }
}
