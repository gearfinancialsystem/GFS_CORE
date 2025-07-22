use lib_actus_events::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_terms::ContractTerms::ContractTerms;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use lib_actus_types::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;

use lib_actus_types::types::IsoDatetime::IsoDatetime;


#[allow(non_camel_case_types)]
pub struct STF_IED_PAM;

impl TraitStateTransitionFunction for STF_IED_PAM {
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
        let contract_role = model.contract_role.as_ref().expect("contract role should be Some");
        let notional_principal = model.notional_principal.as_ref().expect("notionalPrincipal should always be Some");
        let nominal_interest_rate = model.nominal_interest_rate.clone().expect("nominalInterestRate should be Some");
        let notional_principal_s = states.notional_principal.clone().expect("notionalPrincipal should always be Some");
        let nominal_interest_rate_s = states.nominal_interest_rate.clone().expect("nominalInterestRate should be Some");
        
        
        states.notional_principal = NotionalPrincipal::new(contract_role.role_sign() * notional_principal.value()).ok();
        states.nominal_interest_rate = Some(nominal_interest_rate);
        states.status_date = Some(StatusDate::from(*time));

        if let (Some(cycle_anchor_date), Some(initial_exchange_date)) = (
            model.cycle_anchor_date_of_interest_payment.as_ref(),
            model.initial_exchange_date.as_ref(),
        ) {
            if cycle_anchor_date.value() < initial_exchange_date.value() {
                states.accrued_interest = states.accrued_interest.clone().map(|mut accrued_interest| {
                    accrued_interest += notional_principal_s.value() * nominal_interest_rate_s.value() *
                        day_counter.day_count_fraction(
                            time_adjuster.shift_sc(&cycle_anchor_date.value()),
                            time_adjuster.shift_sc(time)
                        );
                    accrued_interest
                });
            }
        }
        
        
    }
}
