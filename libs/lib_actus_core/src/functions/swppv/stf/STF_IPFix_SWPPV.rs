use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_IPFix_SWPPV;

impl TraitStateTransitionFunction for STF_IPFix_SWPPV {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        _model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let status_date = states.status_date.clone().expect("statusDate should always be Some");

        states.last_interest_period = Some(day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.value()),
            time_adjuster.shift_sc(time)
        ));

        states.accrued_interest = Some(0.0);
        states.status_date = Some(StatusDate::from(*time));
    }
}
