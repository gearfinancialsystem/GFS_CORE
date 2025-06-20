use std::os::linux::raw::stat;
use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_TD_PAM;

impl TraitPayOffFunction for POF_TD_PAM {
    fn eval(
        &self,
        time: &IsoDatetime, 
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayConvention,
    ) -> f64 {
        
        let contract_role = model.contractRole.as_ref().expect("contract role should always be some");
        let price_at_termination_date = model.priceAtTerminationDate.expect("priceAtTerminationDate should always exist");
        let accrued_interest = states.accruedInterest.expect("accruedInterest should always exist");
        let status_date = states.statusDate.expect("status date should always exist");
        let nominal_interest_rate = states.nominalInterestRate.expect("nominalInterestRate should be Some");
        let notional_principal = states.notionalPrincipal.expect("notionalPrincipal should always exist");
       
        1.0 *
            contract_role.role_sign() *
            (price_at_termination_date +
            accrued_interest + 
            day_counter.day_count_fraction(
                time_adjuster.shift_bd(&status_date),
                time_adjuster.shift_bd(time)
            ) * nominal_interest_rate
                * notional_principal
            )
        
    }
}
