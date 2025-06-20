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
        
        let interest_scaling_multiplier = states.interestScalingMultiplier.expect("interestScalingMultiplier should always be some");
        let accrued_interest = states.accruedInterest.expect("accruedInterest should always be some");
        let nominal_interest_rate = states.nominalInterestRate.expect("nominalInterestRate should always be some");
        let notional_principal = states.notionalPrincipal.expect("notionalPrincipal should always be some");
        let status_date = states.statusDate.expect("status date should always be some");

        1.0 * interest_scaling_multiplier *
            (accrued_interest +
                day_counter.day_count_fraction(
                    time_adjuster.shift_bd(&status_date),
                    time_adjuster.shift_bd(&time)
                )) * nominal_interest_rate * notional_principal
    }
}
