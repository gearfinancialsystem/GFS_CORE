use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_AD_SWPPV;

impl TraitStateTransitionFunction for STF_AD_SWPPV {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let status_date = states.statusDate.expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominalInterestRate.expect("nominalInterestRate should always be Some");
        let notional_principal = states.notionalPrincipal.expect("notionalPrincipal should always be Some");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date),
            time_adjuster.shift_sc(time)
        );

        let model_nominal_interest_rate = model.nominalInterestRate.unwrap_or(0.0);

        states.accruedInterest = states.accruedInterest.map(|mut accrued_interest| {
            accrued_interest += (model_nominal_interest_rate - nominal_interest_rate) * notional_principal * time_from_last_event;
            accrued_interest += model_nominal_interest_rate * notional_principal * time_from_last_event;
            accrued_interest
        });

        states.accruedInterest2 = states.accruedInterest2.map(|mut accrued_interest2| {
            accrued_interest2 += (-1.0) * nominal_interest_rate * notional_principal * time_from_last_event;
            accrued_interest2
        });

        states.statusDate = Some(*time);
    }
}
