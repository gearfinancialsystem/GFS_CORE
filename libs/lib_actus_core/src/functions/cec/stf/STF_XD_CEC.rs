use crate::attributes::ContractTerms::ContractModel;

use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::terms::grp_contract_identification::contract_types::Cec::CEC;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::terms::grp_settlement::ExerciseAmount::ExerciseAmount;
use crate::terms::grp_settlement::ExerciseDate::ExerciseDate;
use crate::util_tests::essai_data_observer::DataObserver;

#[allow(non_camel_case_types)]
pub struct STF_XD_CEC;

impl TraitStateTransitionFunction for STF_XD_CEC {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &DataObserver,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) {

        let market_value_covering_contracts = CEC::calculate_market_value_covering_contracts(
            model,
            risk_factor_model,
            time
        );
        let a = CEC::calculate_notional_principal(
            model,
            risk_factor_model,
            time
        );
        states.notional_principal = NotionalPrincipal::new(a).ok();

        let exercise_amount = {
            ExerciseAmount::new({
                if states.notional_principal.is_some(){
                    a.min(market_value_covering_contracts)
                }
                else {
                    0.0_f64.min(market_value_covering_contracts)
                    
                }
            }).ok()
        };

            //states.notional_principal.clone().unwrap_or(0.0).min(market_value_covering_contracts);

        states.exercise_amount = exercise_amount;

        states.exercise_date = Some(ExerciseDate::from(*time));
        states.status_date = Some(StatusDate::from(*time));
    }
}
