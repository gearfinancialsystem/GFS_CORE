use std::any::Any;
use std::ops::Deref;
use crate::{contracts::ContractModel::ContractModel, 
            terms::grp_fees::FeeBasis};
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::states::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_fees::fee_basis::A::A;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::PayOffFunctionTrait::PayOffFunctionTrait;


#[allow(non_camel_case_types)]
pub struct POF_FP_PAM;

impl PayOffFunctionTrait for POF_FP_PAM {
    fn eval(
        &self,
        time: IsoDatetime, 
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayConvention,
    ) -> f64 {
        let a = model;
   
        
        if *model.FeeBasis == FeeBasis::FeeBasis::A(A) {
            1.0           // implémenter settlement_currency_fx_rate dans common util
            * model.ContractRole.role_sign() 
            * model.FeeRate.unwrap()
        }
        else {
            1.0                  // implémenter settlement_currency_fx_rate dans common util
            * (states.feeAccrued.as_deref().unwrap()) // + day_counter.day_count_fraction(time_adjuster.shift_sc(&states.statusDate, ), end_time))
            * model.FeeRate.unwrap()
            * states.notionalPrincipal.as_deref().unwrap()
        }

    }
}
