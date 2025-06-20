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

        assert!(model.contractRole.is_some(), "contractRole should always be Some");
        assert!(model.priceAtTerminationDate.is_some(), "priceAtTerminationDate should always be Some");
        assert!(states.accruedInterest.is_some(), "accruedInterest should always be Some");
        assert!(states.statusDate.is_some(), "statusDate should always be Some");
        assert!(states.nominalInterestRate.is_some(), "nominalInterest rate should always be Some");
        assert!(states.notionalPrincipal.is_some(), "notionalPrincipal should always be Some");
        
        let contract_role = model.contractRole.as_ref().unwrap();
        let price_at_termination_date = model.priceAtTerminationDate.unwrap();
        let accrued_interest = states.accruedInterest.unwrap();
        let status_date = states.statusDate.unwrap();
        let nominal_interest_rate = states.nominalInterestRate.unwrap();
        let notional_principal = states.notionalPrincipal.unwrap();
       
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
