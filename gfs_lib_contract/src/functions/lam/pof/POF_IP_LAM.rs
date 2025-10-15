use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
// use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ContractTerms::ContractTerms;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use gfs_lib_types::traits::TraitConvert::IsoDateTimeConvertTo;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::traits::TraitExternalData::TraitExternalData;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct POF_IP_LAM;

impl TraitPayOffFunction for POF_IP_LAM {
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
        let status_date = states.status_date.clone().expect("No status date");
        let interest_scaling_multiplier = states.interest_scaling_multiplier.clone().expect("interest_scaling_multiplier should exist");
        let accrued_interest = states.accrued_interest.clone().expect("accrued_interest should exist");
        let nominal_interest_rate = states.nominal_interest_rate.clone().expect("nominal_interest_rate should exist");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.clone().expect("interest_calculation_base_amount should exist");
        
        
        let timadj = day_counter.day_count_fraction(
            {
                let tmp_sd: PhantomIsoDatetimeW = status_date.convert();
                time_adjuster.shift_sc(&tmp_sd)
            },
            time_adjuster.shift_sc(time)
        );

        settlement_currency_fx_rate * interest_scaling_multiplier.value()
            * (accrued_interest.value() + (timadj * nominal_interest_rate.value() * 
            interest_calculation_base_amount.value())
        )

    }
}
