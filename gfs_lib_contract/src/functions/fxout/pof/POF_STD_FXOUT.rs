use std::sync::Arc;
use gfs_lib_terms::non_terms::PayOff::PayOff;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use crate::attributes::ContractTerms::ContractTerms;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use gfs_lib_types::traits::TraitConvert::IsoDateTimeConvertTo;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::error::error_types::ErrorPayOffComputation::ErrorPayOffComputation;
use crate::error::ErrorContract::ErrorContractEnum;
use crate::traits::TraitExternalData::TraitExternalData;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct POF_STD_FXOUT;

impl TraitPayOffFunction for POF_STD_FXOUT {
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
        let contract_role = contract_terms.contract_role.as_ref().expect("contract role should always exist");
        let contract_role_sign = contract_role.role_sign();
        let notional_principal = contract_terms.notional_principal.clone().expect("notionalPrincipal should always exist");
        let notional_principal_2 = contract_terms.notional_principal2.clone().expect("notionalPrincipal2 should always exist");
        let maturity_date = contract_terms.maturity_date.clone().expect("maturity date should always exist");

        let strings = vec![
                            contract_terms.currency2.clone().unwrap().to_currency(),
                            contract_terms.currency.clone().unwrap()
        ];

        let str_slices: Vec<String> = strings.iter().map(|s| s.value().clone().to_string()).collect();
        let joined = str_slices.join("/");


        let cbv = if let Some(rfm) = risk_factor_external_data {
            rfm.state_at(
                joined,
                &maturity_date.value().convert::<PhantomIsoDatetimeW>(),
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

        let r = settlement_currency_fx_rate *
                          contract_role_sign *
                          (
                                  notional_principal.value() -
                                      cbv.unwrap() *
                                      notional_principal_2.value()
                          );

        match PayOff::new(r) {
            Ok(v) => { Ok(v) },
            Err(e) => {Err(ErrorContractEnum::ErrorPayOffComputation(ErrorPayOffComputation::ErrorTerms(e)))},
        }
    }
}
