
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::attributes::ContractTerms::ContractTerms;
use crate::states_space::StatesSpace::StatesSpace;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::traits::TraitExternalData::TraitExternalData;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct POF_PP_PAM;

impl TraitPayOffFunction for POF_PP_PAM {
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
        
            let notional_principal = states.notional_principal.as_ref().expect("notionalPrincipal should always be some");
            let contract_role = contract_terms.contract_role.as_ref().expect("contract role should always be some");


            let cbv = if let Some(rfm) = risk_factor_external_data {
                rfm.state_at(
                    contract_terms.object_code_of_prepayment_model.clone().unwrap().value(),
                    time,
                )
            } else {
                None
            };

            let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
                risk_factor_external_data,
                contract_terms,
                time,
                states
            );
            settlement_currency_fx_rate * contract_role.role_sign() * cbv.unwrap() * notional_principal.value()

    }
}
