use crate::attributes::ContractReference::ContractReference;
use crate::events::ContractEvent::ContractEvent;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;

use crate::util::CommonUtils::CommonUtils as cu;
use crate::external::RiskFactorModel::RiskFactorModel;
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
        contract_terms: &ContractTerms,
contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<RiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let contract_role = contract_terms.contract_role.as_ref().expect("contract role should always exist");
        let settlement_currency_fx_rate = cu::settlementCurrencyFxRate(
            risk_factor_model,
            contract_terms,
            time,
            states
        );

        settlement_currency_fx_rate
            * contract_role.role_sign()
            * (self.e1.payoff() - self.e2.payoff()).abs()
    }
}
