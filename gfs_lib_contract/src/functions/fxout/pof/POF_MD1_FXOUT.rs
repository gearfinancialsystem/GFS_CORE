use std::sync::Arc;
use gfs_lib_terms::non_terms::PayOff::PayOff;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::error::error_types::ErrorPayOffComputation::ErrorPayOffComputation;
use crate::error::ErrorContract::ErrorContractEnum;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::traits::TraitExternalData::TraitExternalData;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct POF_MD1_FXOUT;

impl TraitPayOffFunction for POF_MD1_FXOUT {
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
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> Result<PayOff, ErrorContractEnum> {
        let contract_role = contract_terms.contract_role.clone().expect("contract role should always exist");
        let contract_role_sign = contract_role.role_sign();
        let notional_principal = contract_terms.notional_principal.clone().expect("notionalPrincipal should always exist");

        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_external_data,
            contract_terms,
            time,
            states
        );
        let r = settlement_currency_fx_rate * contract_role_sign * notional_principal.value();
        match PayOff::new(r) {
            Ok(v) => { Ok(v) },
            Err(e) => {Err(ErrorContractEnum::ErrorPayOffComputation(ErrorPayOffComputation::ErrorTerms(e)))},
        }
    }
}
