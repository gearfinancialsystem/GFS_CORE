use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_IP_SWPPV;

impl TraitStateTransitionFunction for STF_IP_SWPPV {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        _model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        states.accrued_interest = AccruedInterest::new(0.0).ok();
        states.status_date = Some(StatusDate::from(*time));;
    }
}
