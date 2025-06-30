use crate::attributes::ContractModel::ContractModel;
use crate::terms::grp_contract_identification::contract_types::Ceg::CEG;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;


#[allow(non_camel_case_types)]
pub struct STF_FP_CEG;

impl TraitStateTransitionFunction for STF_FP_CEG {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        if model.notionalPrincipal.is_none() {
            states.notionalPrincipal = Some(CEG::calculate_notional_principal(
                states,
                model,
                risk_factor_model,
                time,
            ));
        }

        states.feeAccrued = Some(0.0);
        states.statusDate = Some(*time);
    }
}
