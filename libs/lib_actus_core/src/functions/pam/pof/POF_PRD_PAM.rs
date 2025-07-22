use lib_actus_events::traits::TraitPayOffFunction::TraitPayOffFunction;
use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_types::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;

use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;


#[allow(non_camel_case_types)]
pub struct POF_PRD_PAM;

impl TraitPayOffFunction for POF_PRD_PAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StatesSpace,
        model: &ContractTerms,
        risk_factor_model: Option<&dyn TraitRiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
            let day_counter = day_counter.clone().expect("sould have day counter");
            let contract_role = model.contract_role.as_ref().expect("contract role should always exist");
            let price_at_purchase_date = model.price_at_purchase_date.as_ref().expect("priceAtPurchaseDate should always exist");
            let accrued_interest = model.accrued_interest.as_ref().expect("accruedInterest should always exist");
            let status_date = model.status_date.as_ref().expect("status date should always exist");
            let nominal_interest_rate = model.nominal_interest_rate.as_ref().expect("nominalInterestRate should always exist");
            let notional_principal = model.notional_principal.as_ref().expect("notionalPrincipal should always exist");

            let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
                risk_factor_model,
                model,
                time,
                states
            );
            settlement_currency_fx_rate * contract_role.role_sign() * -1.0 * (
                    price_at_purchase_date.value() + 
                    accrued_interest.value() + day_counter.day_count_fraction(
                    time_adjuster.shift_sc(&status_date.value()),
                    time_adjuster.shift_sc(&time)
                ) * notional_principal.value() * nominal_interest_rate.value())
    }
}
