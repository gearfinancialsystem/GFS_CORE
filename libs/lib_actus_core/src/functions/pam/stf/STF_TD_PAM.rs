use lib_actus_events::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_fees::FeeAccrued::FeeAccrued;
use lib_actus_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;

use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;


#[allow(non_camel_case_types)]
pub struct STF_TD_PAM;

impl TraitStateTransitionFunction for STF_TD_PAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        model: &ContractTerms,
        risk_factor_model: Option<&dyn TraitRiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    )  {

        states.notional_principal = NotionalPrincipal::new(0.0).ok();
        states.accrued_interest = AccruedInterest::new(0.0).ok();
        states.fee_accrued = FeeAccrued::new(0.0).ok();
        states.status_date = Some(StatusDate::from(*time));
        
    }
}
