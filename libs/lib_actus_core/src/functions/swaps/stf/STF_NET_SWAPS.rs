use lib_actus_events::events::ContractEvent::ContractEvent;
use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use lib_actus_events::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;

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
        _model: &ContractTerms,
        risk_factor_model: Option<&dyn TraitRiskFactorModel>,
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