use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_SC_LAM;

impl TraitStateTransitionFunction for STF_SC_LAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let status_date = states.statusDate.expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominalInterestRate.expect("nominalInterestRate should always be Some");
        let interest_calculation_base_amount = states.interestCalculationBaseAmount.expect("interestCalculationBaseAmount should always be Some");
        let notional_principal = states.notionalPrincipal.expect("notionalPrincipal should always be Some");

        let fee_rate = model.feeRate.clone().expect("fee rate should always be Some");
        //let scaling_index_at_contract_deal_date = model.scalingIndexAtContractDealDate.clone().expect("fee rate should always be Some");
        let scaling_effect = model.scalingEffect.clone().expect("fee rate should always be Some");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date),
            time_adjuster.shift_sc(time)
        );

        states.accruedInterest = states.accruedInterest.map(|accrued_interest| {
            accrued_interest + nominal_interest_rate * interest_calculation_base_amount * time_from_last_event
        });

        states.feeAccrued = states.feeAccrued.map(|fee_accrued| {
            let fee_rate = fee_rate;
            fee_accrued + fee_rate * notional_principal * time_from_last_event
        });

        // let market_object_code_of_scaling_index = model.marketObjectCodeOfScalingIndex.as_ref().expect("marketObjectCodeOfScalingIndex should always be Some");


        // let scaling_multiplier = risk_factor_model.state_at(
        //     market_object_code_of_scaling_index,
        //     time,
        //     states,
        //     model,
        //     true
        // ) / scaling_index_at_contract_deal_date;
        let scaling_multiplier = 1.0;


        if scaling_effect.to_string().contains("I") {
            states.interestScalingMultiplier = Some(scaling_multiplier);
        }
        if scaling_effect.to_string().contains("N") {
            states.notionalScalingMultiplier = Some(scaling_multiplier);
        }

        states.statusDate = Some(*time);
    }
}
