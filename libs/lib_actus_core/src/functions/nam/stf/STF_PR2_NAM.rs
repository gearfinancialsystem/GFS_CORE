use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;


#[allow(non_camel_case_types)]
pub struct STF_PR2_NAM;

impl TraitStateTransitionFunction for STF_PR2_NAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let status_date = states.status_date.expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.expect("nominalInterestRate should always be Some");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.expect("interestCalculationBaseAmount should always be Some");
        let notional_principal = states.notional_principal.expect("notionalPrincipal should always be Some");
        let next_principal_redemption_payment = states.next_principal_redemption_payment.expect("nextPrincipalRedemptionPayment should always be Some");
        //let contract_role = model.contract_role.clone().expect("contract role should always be Some");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date),
            time_adjuster.shift_sc(time)
        );

        states.accrued_interest = states.accrued_interest.map(|mut accrued_interest| {
            accrued_interest += nominal_interest_rate * interest_calculation_base_amount * time_from_last_event;
            accrued_interest
        });

        states.fee_accrued = states.fee_accrued.map(|mut fee_accrued| {
            let fee_rate = model.fee_rate.unwrap_or(0.0);
            fee_accrued += fee_rate * notional_principal * time_from_last_event;
            fee_accrued
        });

        let contract_role = model.contract_role.as_ref().expect("contractRole should always be Some");
        let role_sign = contract_role.role_sign();
        let redemption_amount = next_principal_redemption_payment - role_sign * states.accrued_interest.unwrap_or(0.0);
        let redemption = redemption_amount - redemption_amount.max(0.0).min(notional_principal.abs());

        states.notional_principal = Some(states.notional_principal.unwrap_or(0.0) - role_sign * redemption);
        states.interest_calculation_base_amount = states.notional_principal;
        states.status_date = Some(*time);
    }
}
