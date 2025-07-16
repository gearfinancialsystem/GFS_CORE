use crate::attributes::ContractModel::ContractModel;
use crate::terms::grp_contract_identification::contract_types::Ceg::CEG;

use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;

#[allow(non_camel_case_types)]
pub struct STF_FP_CEG;

impl TraitStateTransitionFunction for STF_FP_CEG {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &DataObserver,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        if model.notional_principal.is_none() {
            states.notional_principal = NotionalPrincipal::new( Some(CEG::calculate_notional_principal(
                states,
                model,
                risk_factor_model,
                time,
            )).unwrap()).ok();
        }

        states.fee_accrued = FeeAccrued::new(0.0).ok();
        states.status_date = Some(StatusDate::from(*time));
    }
}
