use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_PRD_PAM;

impl TraitPayOffFunction for POF_PRD_PAM {
    fn eval(
        &self,
        time: &IsoDatetime, 
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        
            let contract_role = model.contract_role.as_ref().expect("contract role should always exist");
            let price_at_purchase_date = model.priceAtPurchaseDate.expect("priceAtPurchaseDate should always exist");
            let accrued_interest = model.accruedInterest.expect("accruedInterest should always exist");
            let status_date = model.statusDate.expect("status date should always exist");
            let nominal_interest_rate = model.nominalInterestRate.expect("nominalInterestRate should always exist");
            let notional_principal = model.notional_principal.expect("notionalPrincipal should always exist");

            let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
                risk_factor_model,
                model,
                time,
                states
            );
            settlement_currency_fx_rate * contract_role.role_sign() * -1.0 * (
                    price_at_purchase_date + 
                    accrued_interest + day_counter.day_count_fraction(
                    time_adjuster.shift_sc(&status_date),
                    time_adjuster.shift_sc(&time)
                ) * notional_principal * nominal_interest_rate)
    }
}
