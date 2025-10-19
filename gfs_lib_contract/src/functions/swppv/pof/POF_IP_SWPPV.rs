use std::sync::Arc;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use gfs_lib_types::traits::TraitConvert::IsoDateTimeConvertToOption;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::traits::TraitExternalData::TraitExternalData;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct POF_IP_SWPPV;

impl TraitPayOffFunction for POF_IP_SWPPV {
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
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_external_data,
            contract_terms,
            time,
            states
        );
        let nominal_interest_rate = contract_terms.nominal_interest_rate.clone().expect("nominalInterestRate should always exist");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&states.status_date.clone().convert_option::<PhantomIsoDatetimeW>().unwrap()),
            time_adjuster.shift_sc(time)
        );

        settlement_currency_fx_rate * (
            states.accrued_interest.clone().unwrap().value() +
                time_from_last_event *
                    (nominal_interest_rate.value() - states.nominal_interest_rate.clone().unwrap().value()) *
                    states.notional_principal.clone().unwrap().value()
        )
    }
}
