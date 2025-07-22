use lib_actus_events::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_types::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use lib_actus_terms::traits::TraitOptionExt::TraitOptionExt;

use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;


#[allow(non_camel_case_types)]
pub struct STF_PY_PAM;
impl TraitStateTransitionFunction for STF_PY_PAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        model: &ContractTerms,
        risk_factor_model: Option<&dyn TraitRiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    )  {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let status_date = states.status_date.as_ref().expect("status date should be some");
        let nominal_interest_rate = states.nominal_interest_rate.as_ref().expect("nominalInterestRate should be some");
        let notional_principal = states.notional_principal.as_ref().expect("notionalPrincipal should be some");
        let fee_rate = model.fee_rate.as_ref().expect("feeRate should be some");
        let time_from_last_event = day_counter.day_count_fraction(time_adjuster.shift_sc(&status_date.value()),
                                                                  time_adjuster.shift_sc(time));

        states.accrued_interest.add_assign(nominal_interest_rate.value() * notional_principal.value() * time_from_last_event);
        states.fee_accrued.add_assign(fee_rate.value() * notional_principal.value() * time_from_last_event);
        states.status_date = Some(StatusDate::from(*time));
        
    }
}
