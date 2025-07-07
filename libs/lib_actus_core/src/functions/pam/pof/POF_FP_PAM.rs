use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;

use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
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
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        
        let fee_basis = model.fee_basis.as_ref().expect("feebasis should always be some");
        let fee_rate = model.fee_rate.expect("fee rate should always be some");
        
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            model,
            time,
            states
        );
        if fee_basis.eq(&FeeBasis::A(A)) {
            let contract_role = model.contract_role.as_ref().expect("contract role should always be some");
            settlement_currency_fx_rate * contract_role.role_sign() * fee_rate
        } 
        else {
            let notional_principal = model.notional_principal.as_ref().expect("notionalPrincipal should always be some");
            let fee_accrued = states.fee_accrued.expect("fee accrued should always be some");
            let status_date = states.status_date.expect("status date should always be some");
            
            settlement_currency_fx_rate * (fee_accrued + day_counter.day_count_fraction(time_adjuster.shift_sc(&status_date), time_adjuster.shift_sc(time))) * fee_rate * notional_principal
        }
        
   
    }

}

