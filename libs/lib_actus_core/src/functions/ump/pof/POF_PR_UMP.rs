use crate::attributes::ContractTerms::ContractModel;

use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitContractModel::TraitContractModel;
use lib_actus_events::traits::TraitPayOffFunction::TraitPayOffFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;
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
        model: &impl TraitContractModel,
        risk_factor_model: &DataObserver,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let contract_role = model.contract_role.as_ref().expect("contract role should always exist");

        contract_role.role_sign() * self.payoff
    }
}
