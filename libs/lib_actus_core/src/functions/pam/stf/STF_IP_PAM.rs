use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_IP_PAM;

impl TraitStateTransitionFunction for STF_IP_PAM {
    fn eval(
        &self,
        time: &IsoDatetime, 
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayConvention,
    ) {
        
        let status_date = states.statusDate.expect("status date should be some");
        let fee_rate = model.feeRate.expect("fee rate should be some");
        let notional_principal = states.notionalPrincipal.expect("notional principal should be some");

        states.accruedInterest = Some(0.0);

        states.feeAccrued = states.feeAccrued.map(|mut fee_accrued| {
            fee_accrued += day_counter.day_count_fraction(time_adjuster.shift_bd(&status_date), time_adjuster.shift_bd(time)) *
                fee_rate * notional_principal;
            fee_accrued
        });

        states.statusDate = Some(*time);
        
    }
}
