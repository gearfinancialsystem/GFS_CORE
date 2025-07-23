use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;

use lib_actus_events::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_terms::traits::TraitOptionExt::TraitOptionExt;
use lib_actus_types::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_PR_UMP {
    payoff: f64,
}

impl STF_PR_UMP {
    pub fn new(event_payoff: f64) -> Self {
        STF_PR_UMP { payoff: event_payoff }
    }
}

impl TraitStateTransitionFunction for STF_PR_UMP {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        model: &ContractTerms,
        _risk_factor_model: Option<&dyn TraitRiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let status_date = states.status_date.clone().expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.clone().expect("nominalInterestRate should always be Some");
        let notional_principal = states.notional_principal.clone().expect("notionalPrincipal should always be Some");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.value()),
            time_adjuster.shift_sc(time)
        );

        states.accrued_interest = states.accrued_interest.clone().map(|mut accrued_interest| {
            accrued_interest += nominal_interest_rate.value() * notional_principal.value() * time_from_last_event;
            accrued_interest
        });

        let contract_role = model.contract_role.clone().expect("contractRole should always be Some");
        let role_sign = contract_role.role_sign();
        states.notional_principal = NotionalPrincipal::new(notional_principal.value() - role_sign * self.payoff).ok();

        states.fee_accrued = states.fee_accrued.clone().map(|mut fee_accrued| {
            let fee_rate = model.fee_rate.clone().itself_or(0.0);
            fee_accrued += fee_rate.value() * states.notional_principal.clone().itself_or(0.0).value() * time_from_last_event;
            fee_accrued
        });

        states.status_date = Some(StatusDate::from(*time));
    }
}
