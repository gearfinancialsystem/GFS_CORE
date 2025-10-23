use std::sync::Arc;
use gfs_lib_terms::non_terms::PayOff::PayOff;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use crate::attributes::ContractTerms::ContractTerms;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use gfs_lib_types::traits::TraitConvert::IsoDateTimeConvertTo;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::error::error_types::ErrorPayOffComputation::ErrorPayOffComputation;
use crate::error::ErrorContract::ErrorContractEnum;
use crate::traits::TraitExternalData::TraitExternalData;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct POF_IPFix_SWPPV;

impl TraitPayOffFunction for POF_IPFix_SWPPV {
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
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_external_data,
            contract_terms,
            time,
            states
        );
        let nominal_interest_rate = contract_terms.nominal_interest_rate.clone().expect("nominalInterestRate should always exist");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&states.status_date.clone().unwrap().convert::<PhantomIsoDatetimeW>()),
            time_adjuster.shift_sc(time)
        );

        let r = settlement_currency_fx_rate * (
            states.accrued_interest.clone().unwrap().value() +
                time_from_last_event *
                    nominal_interest_rate.value()  *
                    states.notional_principal.clone().unwrap().value()
        );
        match PayOff::new(r) {
            Ok(v) => { Ok(v) },
            Err(e) => {Err(ErrorContractEnum::ErrorPayOffComputation(ErrorPayOffComputation::ErrorTerms(e)))},
        }
    }
}
