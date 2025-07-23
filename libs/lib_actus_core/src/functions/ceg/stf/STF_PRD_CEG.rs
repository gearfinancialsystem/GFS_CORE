use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_events::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;

use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_fees::FeeAccrued::FeeAccrued;
use lib_actus_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;

#[allow(non_camel_case_types)]
pub struct STF_PRD_CEG;

impl TraitStateTransitionFunction for STF_PRD_CEG {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        model: &ContractTerms,
        risk_factor_model: Option<&dyn TraitRiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
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
