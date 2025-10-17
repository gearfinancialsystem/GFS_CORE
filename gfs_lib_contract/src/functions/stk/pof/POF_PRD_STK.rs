use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct POF_PRD_STK;

impl TraitPayOffFunction for POF_PRD_STK {
    fn new() -> Self {
        Self {}
    }
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &StatesSpace,
        contract_terms: &ContractTerms,
        _contract_structure: &Option<RelatedContracts>,
        risk_factor_external_data: &Option<Box<dyn TraitExternalData>>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let contract_role = contract_terms.contract_role.as_ref().expect("contract role should always be some");
        let quantity = contract_terms.quantity.clone().expect("quantity should always be some");
        let price_at_purchase_date = contract_terms.price_at_purchase_date.clone().expect("priceAtPurchaseDate should always be some");
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_external_data,
            contract_terms,
            time,
            states
        );
        settlement_currency_fx_rate * contract_role.role_sign() * -1.0 * quantity.value() * price_at_purchase_date.value()
    
    }
}
