use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ContractTerms::ContractTerms;
use crate::contracts::Ceg::CEG;
use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::external::RiskFactorModel::RiskFactorModel;

#[allow(non_camel_case_types)]
pub struct STF_PRD_CEG;

impl TraitStateTransitionFunction for STF_PRD_CEG {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        contract_terms: &ContractTerms,
        contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<RiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        // Set notionalPrincipal if it is not already set
        if contract_terms.notional_principal.is_none() {
            states.notional_principal = NotionalPrincipal::new(Some(
                CEG::calculate_notional_principal(
                    contract_terms,
                    &contract_structure.clone().expect("should be one"),
                    &risk_factor_model.clone().expect("should have one"),
                    time
            )).unwrap()).ok();
        }

        // Set feeAccrued based on feeRate or existing feeAccrued
        if let Some(fee_rate) = contract_terms.fee_rate.clone() {
            states.fee_accrued = FeeAccrued::new(fee_rate.value()).ok();
        } else if let Some(fee_accrued) = contract_terms.fee_accrued.clone() {
            states.fee_accrued = FeeAccrued::new(fee_accrued.value()).ok();
        }
        // TODO: Implement last two possible initializations

        states.status_date = Some(StatusDate::from(*time));
    }
}
