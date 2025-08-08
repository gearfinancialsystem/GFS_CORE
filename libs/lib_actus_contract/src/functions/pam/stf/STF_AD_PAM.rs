
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::attributes::ContractTerms::ContractTerms;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;

use crate::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use lib_actus_terms::traits::TraitOptionExt::TraitOptionExt;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::attributes::ContractReference::ContractReference;
use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
#[allow(non_camel_case_types)]
pub struct STF_AD_PAM;

impl TraitStateTransitionFunction for STF_AD_PAM {
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &mut StatesSpace,
        contract_terms: &ContractTerms,
contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<impl TraitRiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    )  {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let status_date = states.status_date.as_ref().expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.as_ref().expect("nominalInterestRate should always be Some");
        let notional_principal = states.notional_principal.as_ref().expect("notionalPrincipal should always be Some");
        let fee_rate = contract_terms.fee_rate.as_ref().expect("fee rate should be Some");

        let time_from_last_event = day_counter.day_count_fraction(time_adjuster.shift_sc(&status_date.to_phantom_type()),
                                                                        time_adjuster.shift_sc(time));
        
        
        states.accrued_interest.add_assign(nominal_interest_rate.value() * notional_principal.value() * time_from_last_event);
        states.fee_accrued.add_assign(fee_rate.value() * notional_principal.value() * time_from_last_event);
        
        states.status_date = StatusDate::new(time.value()).ok();

    }
}
