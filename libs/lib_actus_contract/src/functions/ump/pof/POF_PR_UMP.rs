use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::attributes::ContractReference::ContractReference;
#[allow(non_camel_case_types)]
pub struct POF_PR_UMP {
    payoff: f64,
}

impl POF_PR_UMP {
    pub fn new(event_payoff: f64) -> Self {
        POF_PR_UMP { payoff: event_payoff }
    }
}

impl TraitPayOffFunction for POF_PR_UMP {
    fn eval(
        &self,
        _time: &IsoDatetime,
        _states: &StatesSpace,
        contract_terms: &ContractTerms,
contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<RiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let contract_role = contract_terms.contract_role.as_ref().expect("contract role should always exist");

        contract_role.role_sign() * self.payoff
    }
}
