use lib_actus_events::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_notional_principal::InterestScalingMultiplier::InterestScalingMultiplier;
use lib_actus_terms::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;
use lib_actus_types::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use lib_actus_terms::traits::TraitOptionExt::TraitOptionExt;

use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;


#[allow(non_camel_case_types)]
pub struct STF_SC_PAM;

impl TraitStateTransitionFunction for STF_SC_PAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        model: &ContractTerms,
        risk_factor_model: Option<&dyn TraitRiskFactorModel>,
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
