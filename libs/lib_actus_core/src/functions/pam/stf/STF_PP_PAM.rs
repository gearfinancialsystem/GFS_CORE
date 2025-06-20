use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_PP_PAM;

impl TraitStateTransitionFunction for STF_PP_PAM {
    fn eval(
        &self,
        time: &IsoDatetime, 
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayConvention,
    ) {

        assert!(states.statusDate.is_some(), "status date some");
        assert!(states.accruedInterest.is_some(), "accrued interest some");
        assert!(states.nominalInterestRate.is_some(), "nominal interest rate");
        assert!(states.notionalPrincipal.is_some(), "notional principal some");
        assert!(model.feeRate.is_some(), "fee rate some");

        assert!(states.feeAccrued.is_some(), "fee accrued some");

        let status_date = states.statusDate.unwrap();
        let accrued_interest = states.accruedInterest.unwrap();
        let nominal_interest_rate = states.nominalInterestRate.unwrap();
        let notional_principal = states.notionalPrincipal.unwrap();
        let fee_rate = model.feeRate.unwrap();


        // Calculate time from the last event
        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_bd(&status_date),
            time_adjuster.shift_bd(&time),
        );

        if let Some(mut accrued_interest) = states.accruedInterest {
            accrued_interest += nominal_interest_rate * notional_principal * time_from_last_event;
            states.accruedInterest = Some(accrued_interest);
        }

        if let Some(mut fee_accrued) = states.feeAccrued {
            fee_accrued += fee_rate * notional_principal * time_from_last_event;
            states.feeAccrued = Some(fee_accrued);
        }
        
        if let Some(mut notional_principal) = states.notionalPrincipal {
            notional_principal -= 1.0 * notional_principal;
            states.notionalPrincipal = Some(notional_principal);
        }

        states.statusDate = Some(*time);


    }
}
