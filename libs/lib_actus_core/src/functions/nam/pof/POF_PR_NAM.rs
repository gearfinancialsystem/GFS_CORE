use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_PR_NAM;

impl TraitPayOffFunction for POF_PR_NAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let contract_role = model.contractRole.as_ref().expect("contract role should always exist");
        let status_date = states.statusDate.as_ref().expect("status date should always exist");
        let next_principal_redemption_payment = states.nextPrincipalRedemptionPayment.expect("nextPrincipalRedemptionPayment should always exist");
        let accrued_interest = states.accruedInterest.expect("accruedInterest should always exist");
        let nominal_interest_rate = states.nominalInterestRate.expect("nominalInterestRate should always exist");
        let interest_calculation_base_amount = states.interestCalculationBaseAmount.expect("interestCalculationBaseAmount should always exist");
        let notional_principal = states.notionalPrincipal.expect("notionalPrincipal should always exist");
        let notional_scaling_multiplier = states.notionalScalingMultiplier.expect("notionalScalingMultiplier should always exist");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date),
            time_adjuster.shift_sc(&time)
        );

        let redemption_amount = next_principal_redemption_payment - contract_role.role_sign() * (accrued_interest + time_from_last_event * nominal_interest_rate * interest_calculation_base_amount);

        let redemption = redemption_amount - redemption_amount.max(0.0) - notional_principal.abs();

        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            model,
            time,
            states
        );
        
        settlement_currency_fx_rate * contract_role.role_sign() * notional_scaling_multiplier * redemption
    }
}
