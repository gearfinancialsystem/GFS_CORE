use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_PRF_ANN;
impl TraitStateTransitionFunction for STF_PRF_ANN {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    )  {

        let status_date = states.status_date.expect("status date should be some");
        let nominal_interest_rate = states.nominal_interest_rate.expect("nominalInterestRate should be some");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.expect("nominalInterestRate should be some");
        let notional_principal = states.notional_principal.expect("notionalPrincipal should be some");
        let fee_rate = model.clone().feeRate.expect("feeRate should be some");
        let contract_role = model.clone().contractRole.expect("contract role should be some");
        
        let time_from_last_event = day_counter.day_count_fraction(time_adjuster.shift_sc(&status_date),
                                                                  time_adjuster.shift_sc(time));

        states.accrued_interest = states.accrued_interest.map(|mut accrued_interest| {
            accrued_interest += nominal_interest_rate * interest_calculation_base_amount * time_from_last_event;
            accrued_interest
        });

        states.fee_accrued = states.fee_accrued.map(|mut fee_accrued| {
            fee_accrued += fee_rate * notional_principal * time_from_last_event;
            fee_accrued
        });

        states.status_date = Some(*time);
        states.next_principal_redemption_payment = Some(contract_role.role_sign() * 1.0); // implementer redemptionm utile

    }
}
