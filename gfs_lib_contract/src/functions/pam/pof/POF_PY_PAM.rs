use std::sync::Arc;
use gfs_lib_terms::non_terms::PayOff::PayOff;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::attributes::ContractTerms::ContractTerms;
use crate::states_space::StatesSpace::StatesSpace;
use gfs_lib_terms::terms::grp_optionality::PenaltyType::PenaltyType;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use gfs_lib_types::traits::TraitConvert::IsoDateTimeConvertTo;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::error::error_types::ErrorPayOffComputation::ErrorPayOffComputation;
use crate::error::ErrorContract::ErrorContractEnum;
use crate::traits::TraitExternalData::TraitExternalData;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct POF_PY_PAM;

impl TraitPayOffFunction for POF_PY_PAM {
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
    ) -> Result<PayOff, ErrorContractEnum> {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let penalty_type = contract_terms.penalty_type.as_ref().expect("penaltyType should be Some");
        let contract_role = contract_terms.contract_role.as_ref().expect("contract role should be Some");

        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_external_data,
            contract_terms,
            time,
            states
        );



        match penalty_type {
            PenaltyType::A(_A) => {
                let penalty_rate = contract_terms.penalty_rate.as_ref().expect("penaltyRate should be Some");
                let r = settlement_currency_fx_rate * contract_role.role_sign() * penalty_rate.value();
                match PayOff::new(r) {
                    Ok(v) => { Ok(v) },
                    Err(e) => {Err(ErrorContractEnum::ErrorPayOffComputation(ErrorPayOffComputation::ErrorTerms(e)))},
                }
            }
            PenaltyType::N(_N) => {
                let penalty_rate = contract_terms.penalty_rate.as_ref().expect("penaltyRate should be Some");
                let status_date = states.status_date.as_ref().expect("status date should always exist");
                let notional_principal = states.notional_principal.as_ref().expect("notionalPrincipal should be Some");

                let r = settlement_currency_fx_rate * contract_role.role_sign()
                    * day_counter.day_count_fraction(time_adjuster.shift_sc(
                    &{
                        let tmp: PhantomIsoDatetimeW = status_date.convert();
                        tmp
                    },
                ), time_adjuster.shift_sc(&time))
                * penalty_rate.value() * notional_principal.value();
                match PayOff::new(r) {
                    Ok(v) => { Ok(v) },
                    Err(e) => {Err(ErrorContractEnum::ErrorPayOffComputation(ErrorPayOffComputation::ErrorTerms(e)))},
                }
            }
            _ => {
                let status_date = states.status_date.as_ref().expect("status date should always exist");
                let notional_principal = states.notional_principal.as_ref().expect("notionalPrincipal should always exist");
                let nominal_interest_rate = states.nominal_interest_rate.as_ref().expect("nominalInterestRate should be Some");
                //let market_object_code_of_rate_reset = contract_terms.marketObjectCodeOfRateReset.as_ref().expect("marketObjectCodeOfRateReset should be Some");
                let cbv = if let Some(rfm) = risk_factor_external_data {
                    rfm.state_at(
                        contract_terms.market_object_code_of_rate_reset.clone().unwrap().value(),
                        time,
                    )
                } else {
                    None
                };
                let r = settlement_currency_fx_rate * contract_role.role_sign()
                    * day_counter.day_count_fraction(time_adjuster.shift_sc(
                    &{
                        let tmp: PhantomIsoDatetimeW = status_date.convert();
                        tmp
                    },
                ), time_adjuster.shift_sc(&time))
                    * notional_principal.value()
                    * 0.0f64.max(nominal_interest_rate.value() - cbv.unwrap());
                match PayOff::new(r) {
                    Ok(v) => { Ok(v) },
                    Err(e) => {Err(ErrorContractEnum::ErrorPayOffComputation(ErrorPayOffComputation::ErrorTerms(e)))},
                }
            }
        }
    }
}
