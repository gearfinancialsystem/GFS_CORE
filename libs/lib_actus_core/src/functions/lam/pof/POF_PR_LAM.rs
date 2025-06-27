use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::isoDatetime::IsoDatetime;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;

#[allow(non_camel_case_types)]
pub struct POF_PR_LAM;

impl TraitPayOffFunction for POF_PR_LAM {
    fn eval(
        &self,
        _time: &IsoDatetime,
        states: &StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let redemption = states.nextPrincipalRedemptionPayment.unwrap()
            - model.clone().contractRole.unwrap().role_sign()
            * (states.nextPrincipalRedemptionPayment.unwrap().abs() - states.notionalPrincipal.unwrap().abs()).max(0.0);

        1.0
            * model.clone().contractRole.unwrap().role_sign()
            * states.notionalPrincipal.unwrap()
            * redemption
    }
}
