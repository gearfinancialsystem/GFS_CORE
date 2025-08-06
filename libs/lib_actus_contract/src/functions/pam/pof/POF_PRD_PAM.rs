
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;

use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::attributes::ContractReference::ContractReference;

#[allow(non_camel_case_types)]
pub struct POF_PRD_PAM;

impl TraitPayOffFunction for POF_PRD_PAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StatesSpace,
        contract_terms: &ContractTerms,
        _contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<RiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
            let day_counter = day_counter.clone().expect("sould have day counter");
            let contract_role = contract_terms.contract_role.as_ref().expect("contract role should always exist");
            let price_at_purchase_date = contract_terms.price_at_purchase_date.as_ref().expect("priceAtPurchaseDate should always exist");
            let accrued_interest = contract_terms.accrued_interest.as_ref().expect("accruedInterest should always exist");
            let status_date = contract_terms.status_date.as_ref().expect("status date should always exist");
            let nominal_interest_rate = contract_terms.nominal_interest_rate.as_ref().expect("nominalInterestRate should always exist");
            let notional_principal = contract_terms.notional_principal.as_ref().expect("notionalPrincipal should always exist");

            let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
                risk_factor_model,
                contract_terms,
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
