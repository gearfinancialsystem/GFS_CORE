use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_IED_PAM;

impl TraitPayOffFunction for POF_IED_PAM {
    fn eval(
        &self,
        _time: &IsoDatetime, 
        _states: &StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayConvention,
    ) -> f64 {
        match (&model.contractRole, &model.notionalPrincipal, &model.premiumDiscountAtIED) {
            (Some(a), Some(b), Some(c)) => 1.0 * -1.0 * a.role_sign() * b * c,
            (a, _, _) => 1.0, // a verifier
        }

    }
}
