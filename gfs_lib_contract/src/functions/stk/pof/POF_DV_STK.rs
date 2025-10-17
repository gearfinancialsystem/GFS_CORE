use std::sync::Arc;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use crate::attributes::ContractTerms::ContractTerms;
use crate::states_space::StatesSpace::StatesSpace;


use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::traits::TraitExternalData::TraitExternalData;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct POF_DV_STK;


impl TraitPayOffFunction for POF_DV_STK {
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
        let contract_role = contract_terms.contract_role.as_ref().expect("contract role should always be some");
        let quantity = contract_terms.quantity.clone().expect("quantity should always be some");
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_external_data,
            contract_terms,
            time,
            states
        );

        // let mut cbv = None;
        if let Some(rfm) = risk_factor_external_data {
            let cbv = rfm.state_at(
                contract_terms.market_object_code_of_dividends.clone().unwrap().value(),
                time,
            );
            // println!("cbv: {:?}", cbv);
            if cbv.is_some() {
                return settlement_currency_fx_rate * contract_role.role_sign() * quantity.value() * cbv.unwrap()
            } else {
                return settlement_currency_fx_rate * contract_role.role_sign() * quantity.value()
            }
        }
        else {
            return settlement_currency_fx_rate * contract_role.role_sign() * quantity.value()
        }


        // settlement_currency_fx_rate * contract_role.role_sign() * quantity.value() * cbv.unwrap()
        
    }
}
