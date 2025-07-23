use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ContractModel::ContractModel;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::terms::grp_settlement::ExerciseAmount::ExerciseAmount;
use crate::terms::grp_settlement::ExerciseDate::ExerciseDate;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::contracts::Cec::CEC;
use crate::traits::TraitContractModel::TraitContractModel;

#[allow(non_camel_case_types)]
pub struct STF_XD_CEC;

impl TraitStateTransitionFunction for STF_XD_CEC {
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
        
        let market_value_covering_contracts =  CEC::calculate_market_value_covering_contracts(
            contract_terms,
            &contract_structure.clone().expect("should be one"),
            &risk_factor_model.clone().expect("should have one"),
            time
        );
        let a = CEC::calculate_notional_principal(
            contract_terms,
            &contract_structure.clone().expect("should be one"),
            &risk_factor_model.clone().expect("should have one"),
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
