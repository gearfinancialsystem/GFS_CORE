use std::os::linux::raw::stat;
use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::isoDatetime::IsoDatetime;

use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_fees::fee_basis::A::A;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_fees::FeeBasis::FeeBasis;

#[allow(non_camel_case_types)]
pub struct POF_FP_PAM;

impl TraitPayOffFunction for POF_FP_PAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayConvention,
    ) -> f64 {
        
        let fee_basis = model.feeBasis.as_ref().expect("feebasis should always be some");
        let fee_rate = model.feeRate.expect("fee rate should always be some");
        
        if fee_basis.eq(&FeeBasis::A(A)) {
            let contract_role = model.contractRole.as_ref().expect("contract role should always be some");
            1.0 * contract_role.role_sign() * fee_rate
        } 
        else {
            let notional_principal = model.notionalPrincipal.as_ref().expect("notionalPrincipal should always be some");
            let fee_accrued = states.feeAccrued.expect("fee accrued should always be some");
            let status_date = states.statusDate.expect("status date should always be some");
            
            1.0 * (fee_accrued + day_counter.day_count_fraction(time_adjuster.shift_bd(&status_date), time_adjuster.shift_bd(time))) * fee_rate * notional_principal
        }
        
   
    }

}

