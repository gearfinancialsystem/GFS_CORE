use crate::events::ContractEvent::ContractEvent;
use crate::attributes::ContractTerms::{ContractTerms};
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::traits::TraitExternalData::TraitExternalData;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct POF_NET_SWAPS {
    pub e1: Option<ContractEvent>,
    pub e2: Option<ContractEvent>,
}

impl POF_NET_SWAPS {
    pub fn new_loaded(e1: ContractEvent, e2: ContractEvent) -> Self {
        POF_NET_SWAPS { e1: Some(e1), e2: Some(e2) }
    }
}

impl TraitPayOffFunction for POF_NET_SWAPS {
    fn new() -> Self {
        Self {
            e1: None,
            e2: None,
        }
    }
    fn eval(
        &self,
        _time: &PhantomIsoDatetimeW,
        _states: &StatesSpace,
        _contract_terms: &ContractTerms,
        _contract_structure: &Option<RelatedContracts>,
        _risk_factor_external_data: &Option<Box<dyn TraitExternalData>>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {

        self.e1.clone().unwrap().payoff.unwrap().value() +
            self.e2.clone().unwrap().payoff.unwrap().value()
    }
}