use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_fees::FeeAccrued::FeeAccrued;
use lib_actus_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_notional_principal::InterestScalingMultiplier::InterestScalingMultiplier;
use lib_actus_terms::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;

use lib_actus_events::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_types::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_SC_LAM;

impl TraitStateTransitionFunction for STF_SC_LAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        model: &ContractTerms,
        risk_factor_model: Option<&dyn TraitRiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let status_date = states.status_date.clone().expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.clone().expect("nominalInterestRate should always be Some");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.clone().expect("interestCalculationBaseAmount should always be Some");
        let notional_principal = states.notional_principal.clone().expect("notionalPrincipal should always be Some");

        let fee_rate_m = model.fee_rate.clone().expect("fee rate should always be Some");
        //let scaling_index_at_contract_deal_date = model.scalingIndexAtContractDealDate.clone().expect("fee rate should always be Some");
        let scaling_effect_m = model.scaling_effect.clone().expect("fee rate should always be Some");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.clone().value()),
            time_adjuster.shift_sc(time)
        );


        states.accrued_interest = AccruedInterest::new({
            states.accrued_interest.clone().unwrap().value() + nominal_interest_rate.value() * interest_calculation_base_amount.value() * time_from_last_event
        }).ok();
        
        states.fee_accrued = FeeAccrued::new({
            states.fee_accrued.clone().unwrap().value() + fee_rate_m.value() * notional_principal.value() * time_from_last_event
        }).ok();
        

        // let market_object_code_of_scaling_index = model.marketObjectCodeOfScalingIndex.as_ref().expect("marketObjectCodeOfScalingIndex should always be Some");


        // let scaling_multiplier = risk_factor_model.state_at(
        //     market_object_code_of_scaling_index,
        //     time,
        //     states,
        //     model,
        //     true
        // ) / scaling_index_at_contract_deal_date;
        let scaling_multiplier = 1.0;


        if scaling_effect_m.to_string().contains("I") {
            states.interest_scaling_multiplier = InterestScalingMultiplier::new(scaling_multiplier).ok();
        }
        if scaling_effect_m.to_string().contains("N") {
            states.notional_scaling_multiplier = NotionalScalingMultiplier::new(scaling_multiplier).ok();
        }

        states.status_date = Some(StatusDate::from(*time));
    }
}
