use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;

use lib_actus_events::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_terms::traits::TraitOptionExt::TraitOptionExt;
use lib_actus_types::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_CD_SWPPV;

impl TraitStateTransitionFunction for STF_CD_SWPPV {
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

        let model_nominal_interest_rate = model.nominal_interest_rate.clone().itself_or(0.0);

        states.accrued_interest = states.accrued_interest.clone().map(|mut accrued_interest| {
            accrued_interest += (model_nominal_interest_rate.value() - nominal_interest_rate.value()) * notional_principal.value() * time_from_last_event;
            accrued_interest += model_nominal_interest_rate.value() * notional_principal.value() * time_from_last_event;
            accrued_interest
        });

        states.accrued_interest2 = states.accrued_interest2.clone().map(|mut accrued_interest2| {
            accrued_interest2 += (-1.0) * nominal_interest_rate.value() * notional_principal.value() * time_from_last_event;
            accrued_interest2
        });

        states.contract_performance = Some(ContractPerformance::new("DF").expect("good cp"));
        states.status_date = Some(StatusDate::from(*time));
    }
}
