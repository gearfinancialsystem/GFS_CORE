use crate::attributes::ContractModel::ContractModel;

use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_notional_principal::InterestScalingMultiplier::InterestScalingMultiplier;
use crate::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::traits::TraitOptionExt::TraitOptionExt;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;
#[allow(non_camel_case_types)]
pub struct STF_SC_LAM;

impl TraitStateTransitionFunction for STF_SC_LAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &DataObserver,
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
