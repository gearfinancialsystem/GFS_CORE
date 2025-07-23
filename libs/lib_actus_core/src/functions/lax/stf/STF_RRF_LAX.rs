use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_fees::FeeAccrued::FeeAccrued;
use lib_actus_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_interest::NominalInterestRate::NominalInterestRate;

use lib_actus_events::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_terms::traits::TraitOptionExt::TraitOptionExt;
use lib_actus_types::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_RRF_LAX {
    scheduled_rate: f64,
}

impl STF_RRF_LAX {
    pub fn new(rate: f64) -> Self {
        STF_RRF_LAX { scheduled_rate: rate }
    }
}

impl TraitStateTransitionFunction for STF_RRF_LAX {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        model: &ContractTerms,
        _risk_factor_model: Option<&dyn TraitRiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let day_counter = day_counter.clone().expect("sould have day counter");
        // Compute new rate
        let rate = self.scheduled_rate.clone() * model.rate_multiplier.clone().itself_or(1.0).value()
            + model.rate_spread.clone().itself_or(0.0).value()
            - states.nominal_interest_rate.clone().itself_or(0.0).value();

        let delta_rate = rate.max(model.period_floor.clone().itself_or(f64::MIN).value()).min(model.period_cap.clone().itself_or(f64::MAX).value());

        let new_rate = (states.nominal_interest_rate.itself_or(0.0).value() + delta_rate)
            .max(model.life_floor.clone().itself_or(f64::MIN).value())
            .min(model.life_cap.itself_or(f64::MAX).value());

        // Update state space
        let status_date = states.status_date.clone().expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.clone().itself_or(0.0);
        let interest_calculation_base_amount = states.interest_calculation_base_amount.clone().itself_or(0.0);

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.value()),
            time_adjuster.shift_sc(time)
        );

        states.accrued_interest = AccruedInterest::new(states.accrued_interest.clone().map(|accrued_interest| {
            accrued_interest.value() + nominal_interest_rate.value() * interest_calculation_base_amount.value() * time_from_last_event
        }).unwrap()).ok();

        states.fee_accrued = FeeAccrued::new(states.fee_accrued.clone().map(|fee_accrued| {
            let fee_rate = model.fee_rate.clone().itself_or(0.0).value();
            fee_accrued.value() + fee_rate * states.notional_principal.clone().itself_or(0.0).value() * time_from_last_event
        }).unwrap()).ok();

        states.nominal_interest_rate = NominalInterestRate::new(new_rate).ok();
        states.status_date = Some(StatusDate::from(*time));
    }
}
