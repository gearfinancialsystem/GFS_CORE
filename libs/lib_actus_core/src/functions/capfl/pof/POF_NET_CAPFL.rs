use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::isoDatetime::IsoDatetime;
use crate::events::ContractEvent::ContractEvent;

#[allow(non_camel_case_types)]
pub struct POF_NET_CAPFL {
    e1: ContractEvent,
    e2: ContractEvent,
}

impl POF_NET_CAPFL {
    pub fn new(e1: ContractEvent, e2: ContractEvent) -> Self {
        POF_NET_CAPFL { e1, e2 }
    }
}

impl TraitPayOffFunction for POF_NET_CAPFL {
    fn eval(
        &self,
        _time: &IsoDatetime,
        _states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let contract_role = model.contractRole.as_ref().expect("contract role should always exist");
        let settlement_currency_fx_rate = 1.0; // Remplacer par 1.0 comme demandé

        settlement_currency_fx_rate
            * contract_role.role_sign()
            * (self.e1.payoff() - self.e2.payoff()).abs()
    }
}
