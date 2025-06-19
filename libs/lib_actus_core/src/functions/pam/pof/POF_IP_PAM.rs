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
        match (states.interestScalingMultiplier, states.accruedInterest, states.nominalInterestRate, states.notionalPrincipal) {
            (Some(a), Some(b), Some(c), Some(d)) => 1.0 * a * (b + day_counter.day_count_fraction(
                time_adjuster.shift_bd(&states.statusDate.unwrap()),
                time_adjuster.shift_bd(&time)
            ) * c * d),
            (a, _, _, _) => 1.0, // a verifier
        }
    }
}
