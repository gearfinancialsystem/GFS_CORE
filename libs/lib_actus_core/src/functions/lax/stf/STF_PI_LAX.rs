use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_PI_LAX {
    pr_payment: f64,
}

impl STF_PI_LAX {
    pub fn new(pr_payment: f64) -> Self {
        STF_PI_LAX { pr_payment }
    }
}

impl TraitStateTransitionFunction for STF_PI_LAX {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let role = &model.clone().contractRole.unwrap().role_sign();
        let redemption = role * self.pr_payment - role * (self.pr_payment.abs() - states.notionalPrincipal.unwrap_or(0.0).abs()).max(0.0);

        let status_date = states.statusDate.expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominalInterestRate.unwrap_or(0.0);
        let interest_calculation_base_amount = states.interestCalculationBaseAmount.unwrap_or(0.0);

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date),
            time_adjuster.shift_sc(time)
        );

        states.accruedInterest = states.accruedInterest.map(|accrued_interest| {
            accrued_interest + nominal_interest_rate * interest_calculation_base_amount * time_from_last_event
        });

        states.feeAccrued = states.feeAccrued.map(|fee_accrued| {
            let fee_rate = model.feeRate.unwrap_or(0.0);
            fee_accrued + fee_rate * states.notionalPrincipal.unwrap_or(0.0) * time_from_last_event
        });

        states.notionalPrincipal = states.notionalPrincipal.map(|notional_principal| {
            notional_principal + redemption
        });

        states.statusDate = Some(*time);
    }
}
