use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;


#[allow(non_camel_case_types)]
pub struct STF_PR2_LAM;

impl TraitStateTransitionFunction for STF_PR2_LAM {
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
        let interest_calculation_base_amount = states.interestCalculationBaseAmount.expect("interestCalculationBaseAmount should always be Some");
        let notional_principal = states.notionalPrincipal.expect("notionalPrincipal should always be Some");
        let next_principal_redemption_payment = states.nextPrincipalRedemptionPayment.expect("nextPrincipalRedemptionPayment should always be Some");
        let contract_role = model.contract_role.clone().expect("contract role should always be Some");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date),
            time_adjuster.shift_sc(time)
        );

        states.accruedInterest = states.accruedInterest.map(|accrued_interest| {
            accrued_interest + nominal_interest_rate * interest_calculation_base_amount * time_from_last_event
        });

        states.feeAccrued = states.feeAccrued.map(|fee_accrued| {
            let fee_rate = model.fee_rate.unwrap_or(0.0);
            fee_accrued + fee_rate * notional_principal * time_from_last_event
        });

        let contract_role = model.contract_role.as_ref().expect("contractRole should always be Some");
        let role_sign = contract_role.role_sign();

        let redemption = next_principal_redemption_payment - role_sign * (next_principal_redemption_payment.abs() - notional_principal.abs()).max(0.0);
        states.notionalPrincipal = Some(notional_principal - role_sign * redemption);
        states.interestCalculationBaseAmount = states.notionalPrincipal;
        states.statusDate = Some(*time);
    }
}
