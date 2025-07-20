use crate::attributes::ContractTerms::ContractTerms;

use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::traits::TraitOptionExt::TraitOptionExt;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;

#[allow(non_camel_case_types)]
pub struct STF_RR_PAM;

impl TraitStateTransitionFunction for STF_RR_PAM {
    fn eval(
        &self,
        time: &IsoDatetime, 
        states: &mut StateSpace,
        model: &ContractTerms,
        _risk_factor_model: &DataObserver,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let rate_multiplier = model.rate_multiplier.as_ref().expect("rate_multiplier should be some");
        let rate_spread = model.rate_spread.as_ref().expect("rate_spread should be some");
        let status_date = states.status_date.as_ref().expect("status date should be some");
        let nominal_interest_rate = states.nominal_interest_rate.as_ref().expect("nominalInterestRate should be some");
        let notional_principal = states.notional_principal.as_ref().expect("notionalPrincipal should be some");
        let period_floor = model.period_floor.as_ref().expect("period floor should be some");
        let period_cap = model.period_cap.as_ref().expect("period cap should be some");
        let life_floor = model.life_floor.as_ref().expect("lifeFloor should be some");
        let life_cap = model.life_cap.as_ref().expect("lifeCap should be some");
        
        let mut rate = 1.0 * rate_multiplier.value() + rate_spread.value();
        let mut delta_rate = rate - nominal_interest_rate.value();

        delta_rate = delta_rate.max(period_floor.value()).min(period_cap.value());
        rate = nominal_interest_rate.value() + delta_rate;
        rate = rate.max(life_floor.value()).min(life_cap.value());

        let time_from_last_event = day_counter.day_count_fraction(time_adjuster.shift_sc(&status_date.value()),
                                                                  time_adjuster.shift_sc(time));
        
        states.accrued_interest.add_assign(nominal_interest_rate.value() * notional_principal.value() * time_from_last_event);
        states.nominal_interest_rate = NominalInterestRate::new(rate).ok(); //Some(rate);

        states.status_date = Some(StatusDate::from(*time));


    }
}
