use std::sync::Arc;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use crate::attributes::ContractTerms::ContractTerms;
use crate::states_space::StatesSpace::StatesSpace;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use gfs_lib_terms::traits::TraitOptionExt::TraitOptionExt;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use gfs_lib_types::traits::TraitConvert::IsoDateTimeConvertTo;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::traits::TraitExternalData::TraitExternalData;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct STF_RR_LAM;

impl TraitStateTransitionFunction for STF_RR_LAM {
    fn new() -> Self {
        Self {}
    }
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &mut StatesSpace,
        contract_terms: &ContractTerms,
        _contract_structure: &Option<RelatedContracts>,
        risk_factor_external_data: &Option<Arc<dyn TraitExternalData>>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let day_counter = day_counter.clone().expect("sould have day counter");
        // let accruedInterest = states.accrued_interest.clone().expect("accruedInterest should always be Some");
        let status_date = states.status_date.clone().expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.clone().expect("nominalInterestRate should always be Some");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.clone().expect("interestCalculationBaseAmount should always be Some");
        let notional_principal = states.notional_principal.clone().expect("notionalPrincipal should always be Some");

        let rate_multiplier_m = contract_terms.rate_multiplier.clone().expect("rateMultiplier should always be Some");
        let rate_spread_m = contract_terms.rate_spread.clone().expect("rateSpread should always be Some");
        let period_floor_m = contract_terms.period_floor.clone().expect("periodFloor should always be Some");
        let period_cap_m = contract_terms.period_cap.clone().expect("periodCap should always be Some");
        let life_floor_m = contract_terms.life_floor.clone().expect("lifeFloor should always be Some");
        let life_cap_m = contract_terms.life_cap.clone().expect("lifeCap should always be Some");
        // let market_object_code_of_rate_reset_m = contract_terms.market_object_code_of_rate_reset.clone().expect("contract_terms.market_object_code_of_rate_reset should be some");
        let fee_rate_m = contract_terms.fee_rate.clone().expect("feeRate should always be Some");


        let time_from_last_event = day_counter.day_count_fraction(
            {
                let tmp : PhantomIsoDatetimeW = status_date.convert();
                time_adjuster.shift_sc(&tmp)
            },
            time_adjuster.shift_sc(time)
        );

        let cbv = if let Some(rfm) = risk_factor_external_data {
            rfm.state_at(
                contract_terms.market_object_code_of_rate_reset.clone().unwrap().value(),
                time,
            )
        } else {
            None
        };

        let mut rate = ( cbv.unwrap() * rate_multiplier_m.value())
            + rate_spread_m.clone().value() - nominal_interest_rate.clone().value();

        let delta_rate = rate.max(period_floor_m.value()).min(period_cap_m.value());
        rate = (nominal_interest_rate.value() + delta_rate).max(life_floor_m.value()).min(life_cap_m.value());

        states.accrued_interest.add_assign(nominal_interest_rate.value() * interest_calculation_base_amount.value() * time_from_last_event);
        states.fee_accrued.add_assign(fee_rate_m.value() * notional_principal.value() * time_from_last_event);


        states.nominal_interest_rate = NominalInterestRate::new(rate).ok();
        states.status_date = StatusDate::new(time.value()).ok();
    }
}
