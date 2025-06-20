use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_SC_PAM;

impl TraitStateTransitionFunction for STF_SC_PAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayConvention,
    ) { // ->StateSpace
        
        assert!(states.statusDate.is_some(), "status Date should always be Some");
        assert!(states.nominalInterestRate.is_some(), "nominal Interest rate should always be Some");
        assert!(states.notionalPrincipal.is_some(), "notional Principal should always be Some");
        assert!(model.feeRate.is_some(), "fee rate should always be Some");
        assert!(model.scalingEffect.is_some(), "scaling effect should always be Some");
        
        // ddd
        assert!(states.accruedInterest.is_some(), "accrued Interest should always be Some");
        assert!(states.feeAccrued.is_some(), "feeAccrued should be None");
    
        let status_date = states.statusDate.unwrap();
        let nominal_interest_rate = states.nominalInterestRate.unwrap();
        let notional_principal = states.notionalPrincipal.unwrap();
        let fee_rate = model.feeRate.unwrap();
        let scaling_effect = model.scalingEffect.as_ref().unwrap();
        
        let time_from_last_event = day_counter.day_count_fraction(time_adjuster.shift_bd(&status_date),
                                                                  time_adjuster.shift_bd(time));

        if let Some(mut accrued_interest) = states.accruedInterest {
            accrued_interest += nominal_interest_rate * notional_principal * time_from_last_event;
            states.accruedInterest = Some(accrued_interest);
        }
        if let Some(mut fee_accrued) = model.feeAccrued {
            fee_accrued += fee_rate * notional_principal * time_from_last_event;
            states.feeAccrued = Some(fee_accrued);
        }
        
        let scaling_multiplier = 1.0; // implementer risk factor
        
        if scaling_effect.to_string().contains("I") {
            states.interestScalingMultiplier = Some(scaling_multiplier);
        }
        if scaling_effect.to_string().contains("N") {
            states.notionalScalingMultiplier = Some(scaling_multiplier);
        }
        
        states.statusDate = Some(*time);
        
    }
}
