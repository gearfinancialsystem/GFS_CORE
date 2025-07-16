use crate::attributes::ContractModel::ContractModel;

use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_notional_principal::InterestScalingMultiplier::InterestScalingMultiplier;
use crate::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::traits::TraitOptionExt::TraitOptionExt;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;
#[allow(non_camel_case_types)]
pub struct STF_SC_PAM;

impl TraitStateTransitionFunction for STF_SC_PAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        _risk_factor_model: &DataObserver,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) { // ->StateSpace
        let day_counter = day_counter.clone().expect("sould have day counter");
        let status_date = states.status_date.as_ref().expect("status date should always be some");
        let nominal_interest_rate = states.nominal_interest_rate.as_ref().expect("nominalInterestRate should always be None");
        let notional_principal = states.notional_principal.as_ref().expect("notionalPrincipal should always be None");
        let fee_rate = model.fee_rate.as_ref().expect("fee rate should always be None");
        let scaling_effect = model.scaling_effect.as_ref().expect("scalingEffect should always be None");
        
        let time_from_last_event = day_counter.day_count_fraction(time_adjuster.shift_sc(&status_date.value()),
                                                                  time_adjuster.shift_sc(time));


        states.accrued_interest.add_assign(nominal_interest_rate.value() * notional_principal.value() * time_from_last_event);
        states.fee_accrued.add_assign(fee_rate.value() * notional_principal.value() * time_from_last_event);
        
        if scaling_effect.to_string().contains("I") {
            states.interest_scaling_multiplier = InterestScalingMultiplier::new(1.0).ok();//
        }
        if scaling_effect.to_string().contains("N") {
            states.notional_scaling_multiplier = NotionalScalingMultiplier::new(1.0).ok();//
        }
        
        states.status_date = Some(StatusDate::from(*time));
        
    }
}
