use crate::{contracts::ContractModel::ContractModel, terms::grp_fees::FeeBasis};
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::states::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::PayOffFunctionTrait::PayOffFunctionTrait;

#[allow(non_camel_case_types)]
pub struct POF_MD_PAM;

impl PayOffFunctionTrait for POF_MD_PAM {
    fn eval(
        &self,
        time: IsoDatetime, 
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayConvention,
    ) -> f64 {
            1.0  // implémenter settlement_currency_fx_rate dans common util
            * states.notionalScalingMultiplier
            * states.notionalPrincipal.as_deref().unwrap()
    }
}
