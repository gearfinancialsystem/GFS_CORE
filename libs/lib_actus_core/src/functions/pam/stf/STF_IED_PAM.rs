use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_IED_PAM;

impl TraitStateTransitionFunction for STF_IED_PAM {
    fn eval(
        &self,
        time: &IsoDatetime, 
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayConvention,
    )  {

        assert!(model.contractRole.is_some(), "Contract role should always be Some");
        assert!(model.notionalPrincipal.is_some(), "notional Principal should always be Some");
        assert!(model.nominalInterestRate.is_some(), "nominalInterest rate should be Some");

        let contract_role = model.contractRole.as_ref().unwrap();
        let notional_principal = model.notionalPrincipal.unwrap();
        let nominal_interest_rate = model.nominalInterestRate.unwrap();


        // ddd
        assert!(states.notionalPrincipal.is_some(), "notional Principal should always be Some");
        assert!(states.nominalInterestRate.is_some(), "nominal Interest rate should always be Some");
        assert!(states.statusDate.is_some(), "status Date should always be Some");
        assert!(states.accruedInterest.is_some(), "accrued Interest should always be Some");
        let notional_principal_s = states.notionalPrincipal.unwrap();
        let nominal_interest_rate_s = states.nominalInterestRate.unwrap();
        let status_date = states.statusDate.unwrap();
        let accrued_interest = states.accruedInterest.unwrap();

        states.notionalPrincipal = Some(contract_role.role_sign() * notional_principal);
        states.nominalInterestRate = Some(nominal_interest_rate);
        states.statusDate = Some(*time);

        if (model.cycleAnchorDateOfInterestPayment.is_some()) &&
            model.cycleAnchorDateOfInterestPayment.unwrap() < model.initialExchangeDate.unwrap() { 
            if let Some(mut accrued_interest) = states.accruedInterest {
                assert!(model.cycleAnchorDateOfInterestPayment.is_some(), "cycleAnchorDateOfInterestPayment should always be Some");
                let cycle_anchor_date_of_interest_payment = model.cycleAnchorDateOfInterestPayment.unwrap();
                accrued_interest += notional_principal_s * nominal_interest_rate_s * 
                    day_counter.day_count_fraction(time_adjuster.shift_bd(&cycle_anchor_date_of_interest_payment),
                                                   time_adjuster.shift_bd(time));
                states.accruedInterest = Some(accrued_interest);
            }
        }
        
        
    }
}
