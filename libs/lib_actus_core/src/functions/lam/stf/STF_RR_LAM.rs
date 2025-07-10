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
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_RR_LAM;

impl TraitStateTransitionFunction for STF_RR_LAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let status_date = states.status_date.clone().expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.clone().expect("nominalInterestRate should always be Some");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.clone().expect("interestCalculationBaseAmount should always be Some");
        let notional_principal = states.notional_principal.clone().expect("notionalPrincipal should always be Some");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.clone().value()),
            time_adjuster.shift_sc(time)
        );

        //let market_object_code_of_rate_reset = model.marketObjectCodeOfRateReset.as_ref().expect("marketObjectCodeOfRateReset should always be Some");
        let rate_multiplier = model.rate_multiplier.clone().expect("rateMultiplier should always be Some");
        let rate_spread = model.rate_spread.clone().expect("rateSpread should always be Some");
        let period_floor = model.period_floor.clone().expect("periodFloor should always be Some");
        let period_cap = model.period_cap.clone().expect("periodCap should always be Some");
        let life_floor = model.life_floor.clone().expect("lifeFloor should always be Some");
        let life_cap = model.life_cap.clone().expect("lifeCap should always be Some");
        // risk_factor_model.state_at(market_object_code_of_rate_reset, time, states, model, true) 
        let rate = ( 1.0 * rate_multiplier.value())
            + rate_spread.clone().value() - nominal_interest_rate.clone().value();

        let delta_rate = rate.max(period_floor.value()).min(period_cap.value());
        let new_rate = (nominal_interest_rate.value() + delta_rate).max(life_floor.value()).min(life_cap.value());


        states.accrued_interest = AccruedInterest::new({
            states.accrued_interest.clone().unwrap().value() + nominal_interest_rate.value() * interest_calculation_base_amount.value() * time_from_last_event
        }).ok();

        states.fee_accrued = FeeAccrued::new({
            let fee_rate = {
                if model.fee_rate.is_none() {
                    0.0
                }
                else { model.fee_rate.clone().unwrap().value() }
            };
            states.fee_accrued.clone().unwrap().value() + fee_rate * {
                if states.notional_principal.is_none() {
                    0.0
                } else {states.notional_principal.clone().unwrap().value()}
            } * time_from_last_event
        }).ok();


        states.nominal_interest_rate = NominalInterestRate::new(new_rate).ok(); 
        states.status_date = Some(StatusDate::from(*time));
    }
}
