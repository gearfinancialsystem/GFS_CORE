use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;
use crate::contracts::CreditEnhancementCollateral::CreditEnhancementCollateral;

#[allow(non_camel_case_types)]
pub struct STF_XD_CEC;

impl TraitStateTransitionFunction for STF_XD_CEC {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayAdjuster,
    ) {

        let market_value_covering_contracts = CreditEnhancementCollateral::calculate_market_value_covering_contracts(
            model,
            risk_factor_model,
            time
        );

        states.notionalPrincipal = Some(CreditEnhancementCollateral::calculate_notional_principal(
            model,
            risk_factor_model,
            time
        ));

        let exercise_amount = states.notionalPrincipal.unwrap_or(0.0).min(market_value_covering_contracts);
        states.exerciseAmount = Some(exercise_amount);

        states.exerciseDate = Some(*time);
        states.statusDate = Some(*time);
    }
}
