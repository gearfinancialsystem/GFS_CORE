use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ContractTerms::ContractTerms;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::types::IsoDatetime::IsoDatetime;

use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;

#[allow(non_snake_case)]
pub trait TraitStateTransitionFunction {

    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        contract_terms: &ContractTerms,
        contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<RiskFactorModel>, //&RiskFactorModel,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    );
}
