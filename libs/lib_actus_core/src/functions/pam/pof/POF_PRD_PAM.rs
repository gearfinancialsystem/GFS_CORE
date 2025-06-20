use std::os::linux::raw::stat;
use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_PRD_PAM;

impl TraitPayOffFunction for POF_PRD_PAM {
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
            assert!(model.priceAtPurchaseDate.is_some(), "priceAtPurchaseDate should always be Some");
            assert!(states.accruedInterest.is_some(), "accruedInterest should always be Some");
            assert!(states.statusDate.is_some(), "statusDate should always be Some");
            assert!(states.nominalInterestRate.is_some(), "nominalInterest rate should be Some");
            assert!(states.notionalPrincipal.is_some(), "notionalPrincipal should be Some");
            
            let contract_role = model.contractRole.as_ref().unwrap();
            let price_at_purchase_date = model.priceAtPurchaseDate.unwrap();
            let accrued_interest = model.accruedInterest.unwrap();
            let status_date = model.statusDate.unwrap();
            let nominal_interest_rate = model.nominalInterestRate.unwrap();
            let notional_principal = model.notionalPrincipal.unwrap();
        
            1.0 * contract_role.role_sign() * -1.0 * (
                    price_at_purchase_date + 
                    accrued_interest + day_counter.day_count_fraction(
                    time_adjuster.shift_bd(&status_date),
                    time_adjuster.shift_bd(&time)
                ) * notional_principal * nominal_interest_rate)
    }
}
