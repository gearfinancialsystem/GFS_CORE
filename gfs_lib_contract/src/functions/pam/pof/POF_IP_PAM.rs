use std::sync::Arc;
use gfs_lib_terms::non_terms::PayOff::PayOff;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use gfs_lib_types::traits::TraitConvert::IsoDateTimeConvertTo;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::error::error_types::ErrorPayOffComputation::ErrorPayOffComputation;
use crate::error::ErrorContract::ErrorContractEnum;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::util::CommonUtils::CommonUtils;
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct POF_IP_PAM;

impl TraitPayOffFunction for POF_IP_PAM {
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

        let day_counter = day_counter.as_ref().expect("sould have day counter");
        let interest_scaling_multiplier = states.interest_scaling_multiplier.as_ref().expect("interestScalingMultiplier should always be some");
        let accrued_interest = states.accrued_interest.as_ref().expect("accruedInterest should always be some");
        let nominal_interest_rate = states.nominal_interest_rate.as_ref().expect("nominalInterestRate should always be some");
        let notional_principal = states.notional_principal.as_ref().expect("notionalPrincipal should always be some");
        let status_date = states.status_date.as_ref().expect("status date should always be some");

        let dcf = day_counter.day_count_fraction(
            {
                let tmp: PhantomIsoDatetimeW = status_date.convert();
                time_adjuster.shift_sc(&tmp)
            },
            time_adjuster.shift_sc(&time.clone())
        );

        let settlement_currency_fx_rate = CommonUtils::settlementCurrencyFxRate(
            risk_factor_external_data,
            contract_terms,
            time,
            states
        );
        
        let r = settlement_currency_fx_rate *
            interest_scaling_multiplier.value() *
            (accrued_interest.value() + dcf *
            nominal_interest_rate.value() *
            notional_principal.value());

        match PayOff::new(r) {
            Ok(v) => { Ok(v) },
            Err(e) => {Err(ErrorContractEnum::ErrorPayOffComputation(ErrorPayOffComputation::ErrorTerms(e)))},
        }

    }
}
