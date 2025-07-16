use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::events::ContractEvent::ContractEvent;
use crate::util::CommonUtils::CommonUtils as cu;
#[allow(non_camel_case_types)]
pub struct POF_NET_CAPFL {
    e1: ContractEvent<IsoDatetime, IsoDatetime>,
    e2: ContractEvent<IsoDatetime, IsoDatetime>,
}

impl POF_NET_CAPFL {
    pub fn new(e1: ContractEvent<IsoDatetime, IsoDatetime>, e2: ContractEvent<IsoDatetime, IsoDatetime>) -> Self {
        POF_NET_CAPFL { e1, e2 }
    }
}

impl TraitPayOffFunction for POF_NET_CAPFL {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let contract_role = model.contract_role.as_ref().expect("contract role should always exist");
        let settlement_currency_fx_rate = cu::settlementCurrencyFxRate(
            risk_factor_model,
            model,
            time,
            states
        );

        settlement_currency_fx_rate
            * contract_role.role_sign()
            * (self.e1.payoff() - self.e2.payoff()).abs()
    }
}
