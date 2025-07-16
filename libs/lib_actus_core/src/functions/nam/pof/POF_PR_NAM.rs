use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_PR_NAM;

impl TraitPayOffFunction for POF_PR_NAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let contract_role = model.contract_role.clone().expect("contract role should always exist");
        let status_date = states.status_date.clone().expect("status date should always exist");
        let next_principal_redemption_payment = states.next_principal_redemption_payment.clone().expect("nextPrincipalRedemptionPayment should always exist");
        let accrued_interest = states.accrued_interest.clone().expect("accruedInterest should always exist");
        let nominal_interest_rate = states.nominal_interest_rate.clone().expect("nominalInterestRate should always exist");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.clone().expect("interestCalculationBaseAmount should always exist");
        let notional_principal = states.notional_principal.clone().expect("notionalPrincipal should always exist");
        let notional_scaling_multiplier = states.notional_scaling_multiplier.clone().expect("notionalScalingMultiplier should always exist");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.value()),
            time_adjuster.shift_sc(&time)
        );

        let redemption_amount = next_principal_redemption_payment.value() - 
            contract_role.role_sign() * 
                (accrued_interest.value() + time_from_last_event * nominal_interest_rate.value() * interest_calculation_base_amount.value());

        let redemption = redemption_amount - 0.0_f64.max(redemption_amount - notional_principal.value().abs());

        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            model,
            time,
            states
        );
        
        settlement_currency_fx_rate * 
            contract_role.role_sign() * 
            notional_scaling_multiplier.value() * redemption
    }
}
