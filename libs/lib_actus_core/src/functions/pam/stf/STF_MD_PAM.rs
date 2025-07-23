use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;

use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::types::IsoDatetime::IsoDatetime;


use crate::external::RiskFactorModel::RiskFactorModel;
use crate::attributes::ContractReference::ContractReference;

#[allow(non_camel_case_types)]
pub struct STF_MD_PAM;

impl TraitStateTransitionFunction for STF_MD_PAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        contract_terms: &ContractTerms,
contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<RiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        // let mut new_states: StateSpace = states.copy_state_space(); 
        // Set state values to zero at maturity
        states.notional_principal = NotionalPrincipal::new(0.0).ok();   //Some(0.0);
        states.accrued_interest = AccruedInterest::new(0.0).ok();
        states.fee_accrued = FeeAccrued::new(0.0).ok();
        
        // Update the status date
        states.status_date = Some(StatusDate::from(*time));
        
    }
}
