use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_events::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;

use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use lib_actus_terms::terms::grp_settlement::ExerciseAmount::ExerciseAmount;
use lib_actus_terms::terms::grp_settlement::ExerciseDate::ExerciseDate;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use crate::contracts::Cec::CEC;

#[allow(non_camel_case_types)]
pub struct STF_XD_CEC;

impl TraitStateTransitionFunction for STF_XD_CEC {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        model: &ContractTerms,
        risk_factor_model: Option<&dyn TraitRiskFactorModel>,
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
