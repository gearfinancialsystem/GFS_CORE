use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::terms::grp_contract_identification::contract_types::Ceg::CEG;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;

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
            states.notional_principal = NotionalPrincipal::new(Some(CEG::calculate_notional_principal(
                states,
                model,
                risk_factor_model,
                time,
            )).unwrap()).ok();
        }

        // Set feeAccrued based on feeRate or existing feeAccrued
        if let Some(fee_rate) = model.fee_rate.clone() {
            states.fee_accrued = FeeAccrued::new(fee_rate.value()).ok();
        } else if let Some(fee_accrued) = model.fee_accrued.clone() {
            states.fee_accrued = FeeAccrued::new(fee_accrued.value()).ok();
        }
        // TODO: Implement last two possible initializations

        states.status_date = Some(StatusDate::from(*time));
    }
}
