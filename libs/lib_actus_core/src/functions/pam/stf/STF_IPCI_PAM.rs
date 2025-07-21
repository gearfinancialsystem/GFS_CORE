use crate::attributes::ContractTerms::ContractTerms;
use crate::external::RiskFactors::RiskFactors;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::traits::TraitOptionExt::TraitOptionExt;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_IPCI_PAM;

impl TraitStateTransitionFunction for STF_IPCI_PAM {
    fn eval(
        &self,
        time: &IsoDatetime, 
        states: &mut StateSpace,
        model: &ContractTerms,
        _risk_factor_model: &RiskFactors,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    )  {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let status_date = states.status_date.as_ref().expect("statusDate should always be Some");
        let accrued_interest = states.accrued_interest.as_ref().expect("accruedInterest should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.as_ref().expect("nominalInterestRate should always be Some");
        let notional_principal = states.notional_principal.clone().expect("notionalPrincipal should always be Some");
        
        let fee_rate = model.fee_rate.as_ref().expect("fee rate should always be Some");
        
        // Calculate time from the last event
        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.value()),
            time_adjuster.shift_sc(&time),
        );

        states.notional_principal.add_assign(accrued_interest.value() + (nominal_interest_rate.value() * notional_principal.value() * time_from_last_event));

        states.accrued_interest = AccruedInterest::new(0.0).ok();
        states.fee_accrued.add_assign(fee_rate.value() * notional_principal.value() * time_from_last_event);

        
    }
}
