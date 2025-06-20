use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;


#[allow(non_camel_case_types)]
pub struct STF_RR_PAM;

impl TraitStateTransitionFunction for STF_RR_PAM {
    fn eval(
        &self,
        time: &IsoDatetime, 
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayConvention,
    ) {
        assert!(model.rateMultiplier.is_some(), "rateMultiplier should always be None");
        assert!(model.rateSpread.is_some(), "rateSpread should always be None");

        assert!(states.statusDate.is_some(), "status Date should always be Some");
        assert!(states.nominalInterestRate.is_some(), "nominal Interest rate should always be Some");
        assert!(states.notionalPrincipal.is_some(), "notional Principal should always be Some");

        assert!(model.periodFloor.is_some(), "periodFloor should always be None");
        assert!(model.periodCap.is_some(), "periodCap should be Some");

        assert!(model.lifeFloor.is_some(), "lifeFloor should always be None");
        assert!(model.lifeCap.is_some(), "lifeCap should be Some");

        // ddd
        assert!(states.accruedInterest.is_some(), "accrued Interest should always be Some");
        assert!(states.feeAccrued.is_some(), "feeAccrued should be None");

        let rate_multiplier = model.rateMultiplier.unwrap();
        let rate_spread = model.rateSpread.unwrap();
        let status_date = states.statusDate.unwrap();
        let nominal_interest_rate = states.nominalInterestRate.unwrap();
        let notional_principal = states.notionalPrincipal.unwrap();
        let period_floor = model.periodFloor.unwrap();
        let period_cap = model.periodCap.unwrap();
        let life_floor = model.lifeFloor.unwrap();
        let life_cap = model.lifeCap.unwrap();

        ////aaaaaaa
        let mut rate = 1.0 * rate_multiplier + rate_spread;
        let mut delta_rate = rate - nominal_interest_rate;

        delta_rate = delta_rate.max(period_floor).min(period_cap);
        rate = nominal_interest_rate + delta_rate;
        rate = rate.max(life_floor).min(life_cap);

        let time_from_last_event = day_counter.day_count_fraction(time_adjuster.shift_bd(&status_date),
                                                                  time_adjuster.shift_bd(time));

        if let Some(mut accrued_interest) = states.accruedInterest {
            accrued_interest += nominal_interest_rate * notional_principal * time_from_last_event;
            states.accruedInterest = Some(accrued_interest);
        }
        states.nominalInterestRate = Some(rate);

        states.statusDate = Some(*time);


    }
}
