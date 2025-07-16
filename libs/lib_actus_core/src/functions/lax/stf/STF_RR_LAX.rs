use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::traits::TraitOptionExt::TraitOptionExt;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_RR_LAX {
    scheduled_rate: f64,
}

impl STF_RR_LAX {
    pub fn new(rate: f64) -> Self {
        STF_RR_LAX { scheduled_rate: rate }
    }
}

impl TraitStateTransitionFunction for STF_RR_LAX {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let day_counter = day_counter.clone().expect("sould have day counter");
        // Compute new rate
        let rate = (1.0 * model.rate_multiplier.clone().itself_or(1.0).value()) // Placeholder for risk_factor_model logic
            + model.rate_spread.clone().itself_or(0.0).value()
            + self.scheduled_rate
            - states.nominal_interest_rate.itself_or(0.0).value();

        let delta_rate = rate.max(model.period_floor.itself_or(f64::MIN).value()).min(model.period_cap.itself_or(f64::MAX).value());

        let new_rate = (states.nominal_interest_rate.itself_or(0.0).value() + delta_rate)
            .max(model.life_floor.itself_or(f64::MIN).value())
            .min(model.life_cap.itself_or(f64::MAX).value());

        // Update state space
        let status_date = states.status_date.clone().expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.itself_or(0.0);
        let interest_calculation_base_amount = states.interest_calculation_base_amount.itself_or(0.0);

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.value()),
            time_adjuster.shift_sc(time)
        );

        states.accrued_interest = AccruedInterest::new(states.accrued_interest.clone().map(|accrued_interest| {
            accrued_interest.value() + nominal_interest_rate.value() * interest_calculation_base_amount.value() * time_from_last_event
        }).unwrap()).ok();

        states.fee_accrued = FeeAccrued::new(states.fee_accrued.clone().map(|fee_accrued| {
            let fee_rate = model.fee_rate.itself_or(0.0);
            fee_accrued.value() + fee_rate.value() * states.notional_principal.itself_or(0.0).value() * time_from_last_event
        }).unwrap()).ok();

        states.nominal_interest_rate = NominalInterestRate::new(new_rate).ok();
        states.status_date = Some(StatusDate::from(*time));
    }
}
