use std::sync::Arc;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;

use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use gfs_lib_types::traits::TraitConvert::IsoDateTimeConvertTo;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::traits::TraitExternalData::TraitExternalData;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct POF_PRD_PAM;

impl TraitPayOffFunction for POF_PRD_PAM {
    fn new() -> Self {
        Self {}
    }
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &StatesSpace,
        contract_terms: &ContractTerms,
        _contract_structure: &Option<RelatedContracts>,
        risk_factor_external_data: &Option<Arc<dyn TraitExternalData>>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
            let day_counter = day_counter.clone().expect("sould have day counter");
            let contract_role = contract_terms.contract_role.as_ref().expect("contract role should always exist");
            let price_at_purchase_date = contract_terms.price_at_purchase_date.as_ref().expect("priceAtPurchaseDate should always exist");
            let accrued_interest = states.accrued_interest.as_ref().expect("accruedInterest should always exist");
            let status_date = states.status_date.as_ref().expect("status date should always exist");
            let nominal_interest_rate = states.nominal_interest_rate.as_ref().expect("nominalInterestRate should always exist");
            let notional_principal = states.notional_principal.as_ref().expect("notionalPrincipal should always exist");

            let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
                risk_factor_external_data,
                contract_terms,
                time,
                states
            );
            settlement_currency_fx_rate *
                contract_role.role_sign() *
                -1.0 * (
                    price_at_purchase_date.value() + 
                    accrued_interest.value() + day_counter.day_count_fraction(
                    time_adjuster.shift_sc(
                        &{
                            let tmp: PhantomIsoDatetimeW = status_date.convert();
                            tmp
                        },
                    ),
                    time_adjuster.shift_sc(&time)
                ) * notional_principal.value() * nominal_interest_rate.value())
    }
}
