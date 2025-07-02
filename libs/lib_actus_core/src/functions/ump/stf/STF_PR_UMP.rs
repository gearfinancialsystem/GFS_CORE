use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_PR_UMP {
    payoff: f64,
}

impl STF_PR_UMP {
    pub fn new(event_payoff: f64) -> Self {
        STF_PR_UMP { payoff: event_payoff }
    }
}

impl TraitStateTransitionFunction for STF_PR_UMP {
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

        states.accruedInterest = states.accruedInterest.map(|mut accrued_interest| {
            accrued_interest += nominal_interest_rate * notional_principal * time_from_last_event;
            accrued_interest
        });

        let contract_role = model.contract_role.clone().expect("contractRole should always be Some");
        let role_sign = contract_role.role_sign();
        states.notionalPrincipal = Some(notional_principal - role_sign * self.payoff);

        states.feeAccrued = states.feeAccrued.map(|mut fee_accrued| {
            let fee_rate = model.fee_rate.unwrap_or(0.0);
            fee_accrued += fee_rate * states.notionalPrincipal.unwrap_or(0.0) * time_from_last_event;
            fee_accrued
        });

        states.statusDate = Some(*time);
    }
}
