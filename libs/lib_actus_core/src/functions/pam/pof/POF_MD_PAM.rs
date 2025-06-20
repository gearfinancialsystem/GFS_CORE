use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_MD_PAM;

impl TraitPayOffFunction for POF_MD_PAM {
    fn eval(
        &self,
        _time: &IsoDatetime,
        states: &StateSpace,
        _model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayConvention,
    ) -> f64 {
        
            let notional_scaling_multiplier = states.notionalScalingMultiplier.expect("notionalScalingMultiplier should always be some");
            let notional_principal = states.notionalPrincipal.expect("notionalPrincipal should always be some");
        
            1.0 * notional_scaling_multiplier * notional_principal
        
    }
}
