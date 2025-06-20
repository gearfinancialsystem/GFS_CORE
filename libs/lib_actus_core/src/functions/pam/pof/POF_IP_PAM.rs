use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_IP_PAM;

impl TraitPayOffFunction for POF_IP_PAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StateSpace,
        _model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayConvention,
    ) -> f64 {
        
        assert!(states.interestScalingMultiplier.is_some(), "interest Scaling Multiplier should always be Some");
        assert!(states.accruedInterest.is_some(), "accrued Interest should always be Some");
        assert!(states.nominalInterestRate.is_some(), "nominal Interest rate should always be Some");
        assert!(states.notionalPrincipal.is_some(), "notional Principal should always be Some");
        assert!(states.statusDate.is_some(), "status Date should always be Some");
        
        let interest_scaling_multiplier = states.interestScalingMultiplier.unwrap();
        let accrued_interest = states.accruedInterest.unwrap();
        let nominal_interest_rate = states.nominalInterestRate.unwrap();
        let notional_principal = states.notionalPrincipal.unwrap();
        let status_date = states.statusDate.unwrap();

        1.0 * interest_scaling_multiplier *
            (accrued_interest +
                day_counter.day_count_fraction(
                    time_adjuster.shift_bd(&status_date),
                    time_adjuster.shift_bd(&time)
                )) * nominal_interest_rate * notional_principal
    }
}
