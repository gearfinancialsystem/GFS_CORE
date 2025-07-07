use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::terms::grp_contract_identification::contract_types::Ceg::CEG;

#[allow(non_camel_case_types)]
pub struct STF_PRD_CEG;

impl TraitStateTransitionFunction for STF_PRD_CEG {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        // Set notionalPrincipal if it is not already set
        if model.notional_principal.is_none() {
            states.notional_principal = Some(CEG::calculate_notional_principal(
                states,
                model,
                risk_factor_model,
                time,
            ));
        }

        // Set feeAccrued based on feeRate or existing feeAccrued
        if let Some(fee_rate) = model.fee_rate {
            states.fee_accrued = Some(fee_rate);
        } else if let Some(fee_accrued) = model.fee_accrued {
            states.fee_accrued = Some(fee_accrued);
        }
        // TODO: Implement last two possible initializations

        states.status_date = Some(*time);
    }
}
