use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_fees::FeeAccrued::FeeAccrued;
use lib_actus_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_notional_principal::InterestScalingMultiplier::InterestScalingMultiplier;
use lib_actus_terms::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;

use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;

use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;

#[allow(non_camel_case_types)]
pub struct STF_SC_LAM;

impl TraitStateTransitionFunction for STF_SC_LAM {
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &mut StatesSpace,
        contract_terms: &ContractTerms,
        _contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<impl TraitRiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let status_date = states.status_date.clone().expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.clone().expect("nominalInterestRate should always be Some");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.clone().expect("interestCalculationBaseAmount should always be Some");
        let notional_principal = states.notional_principal.clone().expect("notionalPrincipal should always be Some");

        let fee_rate_m = contract_terms.fee_rate.clone().expect("fee rate should always be Some");
        //let scaling_index_at_contract_deal_date = contract_terms.scalingIndexAtContractDealDate.clone().expect("fee rate should always be Some");
        let scaling_effect_m = contract_terms.scaling_effect.clone().expect("fee rate should always be Some");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.clone().to_phantom_type()),
            time_adjuster.shift_sc(time)
        );


        states.accrued_interest = AccruedInterest::new({
            states.accrued_interest.clone().unwrap().value() + nominal_interest_rate.value() * interest_calculation_base_amount.value() * time_from_last_event
        }).ok();
        
        states.fee_accrued = FeeAccrued::new({
            states.fee_accrued.clone().unwrap().value() + fee_rate_m.value() * notional_principal.value() * time_from_last_event
        }).ok();
        

        // let market_object_code_of_scaling_index = contract_terms.marketObjectCodeOfScalingIndex.as_ref().expect("marketObjectCodeOfScalingIndex should always be Some");


        let mut cbv = None;
        if let Some(rfm) = risk_factor_model {
            cbv = rfm.state_at(
                contract_terms.market_object_code_of_scaling_index.clone().unwrap().value(),
                time,
                states,
                contract_terms,
                true
            );
        } else {
            cbv = None
        }



        if scaling_effect_m.to_string().contains("I") {
            states.interest_scaling_multiplier = InterestScalingMultiplier::new(cbv.unwrap()).ok();
        }
        if scaling_effect_m.to_string().contains("N") {
            states.notional_scaling_multiplier = NotionalScalingMultiplier::new(cbv.unwrap()).ok();
        }

        states.status_date = StatusDate::new(time.value()).ok();
    }
}
