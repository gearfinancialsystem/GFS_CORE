use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;

use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_fees::FeeAccrued::FeeAccrued;
use lib_actus_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use lib_actus_types::types::IsoDatetime::IsoDatetime;



// use crate::attributes::ContractReference::ContractReference;
use crate::traits::_TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::traits::TraitExternalData::TraitExternalData;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct STF_MD_PAM;

impl TraitStateTransitionFunction for STF_MD_PAM {
    fn new() -> Self {
        Self {}
    }
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &mut StatesSpace,
        _contract_terms: &ContractTerms,
        _contract_structure: &Option<RelatedContracts>,
        _risk_factor_external_data: &Option<Box<dyn TraitExternalData>>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        // let mut new_states: StateSpace = states.copy_state_space(); 
        // Set state values to zero at maturity
        states.notional_principal = NotionalPrincipal::new(0.0).ok();   //Some(0.0);
        states.accrued_interest = AccruedInterest::new(0.0).ok();
        states.fee_accrued = FeeAccrued::new(0.0).ok();
        
        // Update the status date
        states.status_date = StatusDate::new(time.value()).ok();
        
    }
}
