use lib_actus_events::events::ContractEvent::ContractEvent;
use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_events::traits::TraitPayOffFunction::TraitPayOffFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;

use crate::util::CommonUtils::CommonUtils as cu;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;
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
        states: &StatesSpace,
        model: &ContractTerms,
        risk_factor_model: Option<&dyn TraitRiskFactorModel>,
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
