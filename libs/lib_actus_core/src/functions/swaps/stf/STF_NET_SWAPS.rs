use crate::events::ContractEvent::ContractEvent;
use crate::attributes::ContractTerms::ContractTerms;

use crate::external::RiskFactorModel::RiskFactorModel;
use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::attributes::ContractReference::ContractReference;
#[allow(non_camel_case_types)]
pub struct STF_NET_SWAPS {
    pub e1: Option<ContractEvent<IsoDatetime, IsoDatetime>>,
    pub e2: Option<ContractEvent<IsoDatetime, IsoDatetime>>,
}

impl STF_NET_SWAPS {
    pub fn new(e1: ContractEvent<IsoDatetime, IsoDatetime>, e2: ContractEvent<IsoDatetime, IsoDatetime>) -> Self {
        STF_NET_SWAPS { e1: Some(e1), e2: Some(e2) }
    }
}

impl TraitStateTransitionFunction for STF_NET_SWAPS {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        _contract_terms: &ContractTerms,
        _contract_structure: &Option<Vec<ContractReference>>,
        _risk_factor_model: &Option<RiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        let e1_states = self.e1.clone().unwrap().states();
        let e2_states = self.e2.clone().unwrap().states();

        let notional_principal_e1 = e1_states.notional_principal.expect("should be some");
        let notional_principal_e2 = e2_states.notional_principal.expect("should be some");
        let accrued_interest_e1 = e1_states.accrued_interest.expect("should be some");
        let accrued_interest_e2 = e2_states.accrued_interest.expect("should be some");

        states.notional_principal = NotionalPrincipal::new(notional_principal_e1.value() + notional_principal_e2.value()).ok();
        states.accrued_interest = AccruedInterest::new(accrued_interest_e1.value() + accrued_interest_e2.value()).ok();

        states.status_date = Some(StatusDate::from(*time));
    }
}