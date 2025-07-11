use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

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
        states: &mut StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        // Compute new rate
        let rate = self.scheduled_rate.clone() * model.rate_multiplier.clone().unwrap_or(1.0)
            + model.rate_spread.clone().unwrap_or(0.0)
            - states.nominal_interest_rate.clone().unwrap_or(0.0);

        let delta_rate = rate.max(model.period_floor.clone().unwrap_or(f64::MIN)).min(model.period_cap.clone().unwrap_or(f64::MAX));

        let new_rate = (states.nominal_interest_rate.unwrap_or(0.0) + delta_rate)
            .max(model.life_floor.clone().unwrap_or(f64::MIN))
            .min(model.life_cap.unwrap_or(f64::MAX));

        // Update state space
        let status_date = states.status_date.clone().expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.clone().unwrap_or(0.0);
        let interest_calculation_base_amount = states.interest_calculation_base_amount.clone().unwrap_or(0.0);

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date),
            time_adjuster.shift_sc(time)
        );

        states.accrued_interest = states.accrued_interest.clone().map(|accrued_interest| {
            accrued_interest + nominal_interest_rate * interest_calculation_base_amount * time_from_last_event
        });

        states.fee_accrued = states.fee_accrued.clone().map(|fee_accrued| {
            let fee_rate = model.fee_rate.clone().unwrap_or(0.0);
            fee_accrued + fee_rate * states.notional_principal.clone().unwrap_or(0.0) * time_from_last_event
        });

        states.nominal_interest_rate = NominalInterestRate::new(new_rate).ok();
        states.status_date = Some(StatusDate::from(*time));
    }
}
