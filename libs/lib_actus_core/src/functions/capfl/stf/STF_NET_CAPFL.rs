use crate::attributes::ContractTerms::ContractModel;

use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::events::ContractEvent::ContractEvent;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::util_tests::essai_data_observer::DataObserver;
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
        time: &IsoDatetime,
        states: &mut StateSpace,
        _model: &ContractModel,
        _risk_factor_model: &DataObserver,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    )  {
        states.status_date = Some(StatusDate::from(*time));
    }
}
