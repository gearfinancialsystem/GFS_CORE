use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

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
        time_adjuster: &BusinessDayAdjuster,
    ) {
        
        let status_date = states.status_date.expect("status date should be some");
        let fee_rate = model.fee_rate.expect("fee rate should be some");
        let notional_principal = states.notional_principal.expect("notional principal should be some");

        states.accrued_interest = Some(0.0);

        states.fee_accrued = states.fee_accrued.map(|mut fee_accrued| {
            fee_accrued += day_counter.day_count_fraction(time_adjuster.shift_sc(&status_date), time_adjuster.shift_sc(time)) *
                fee_rate * notional_principal;
            fee_accrued
        });

        states.status_date = Some(*time);
        
    }
}
