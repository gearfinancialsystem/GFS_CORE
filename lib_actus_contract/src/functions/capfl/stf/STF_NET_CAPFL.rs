use crate::attributes::ContractReference::ContractReference;
use crate::events::ContractEvent::ContractEvent;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;

use crate::types::IsoDatetime::IsoDatetime;

use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;

#[allow(non_camel_case_types)]
pub struct STF_NET_CAPFL {
    e1: ContractEvent<IsoDatetime, IsoDatetime>,
    e2: ContractEvent<IsoDatetime, IsoDatetime>,
}

impl STF_NET_CAPFL {
    pub fn new(e1: ContractEvent<IsoDatetime, IsoDatetime>, e2: ContractEvent<IsoDatetime, IsoDatetime>) -> Self {
        STF_NET_CAPFL { e1, e2 }
    }
}

impl TraitStateTransitionFunction for STF_NET_CAPFL {
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &mut StatesSpace,
        _contract_terms: &ContractTerms,
contract_structure: &Option<Vec<ContractReference>>,
        _risk_factor_model: &Option<impl TraitRiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    )  {
        states.status_date = StatusDate::new(time.value()).ok();
    }
}
