use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;


use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use lib_actus_terms::traits::TraitOptionExt::TraitOptionExt;
use lib_actus_types::types::IsoDatetime::IsoDatetime;



use crate::attributes::ContractReference::ContractReference;

use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;

#[allow(non_camel_case_types)]
pub struct STF_PP_PAM;

impl TraitStateTransitionFunction for STF_PP_PAM {
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &mut StatesSpace,
        contract_terms: &ContractTerms,
        _contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<impl TraitRiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let status_date = states.status_date.as_ref().expect("status date should be some");
        let nominal_interest_rate = states.nominal_interest_rate.as_ref().expect("nominalInterestRate should be some");
        let notional_principal = states.notional_principal.as_ref().expect("notionalPrincipal should be some");
        let fee_rate = contract_terms.fee_rate.as_ref().expect("fee rate should be some");
        
        // Calculate time from the last event
        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.to_phantom_type()),
            time_adjuster.shift_sc(&time),
        );

        let mut cbv = None;
        if let Some(rfm) = risk_factor_model {
            cbv = rfm.state_at(
                contract_terms.object_code_of_prepayment_model.clone().unwrap().value(),
                time,
                states,
                contract_terms,
                true
            );
        } else {
            cbv = None
        }

        states.accrued_interest.add_assign(nominal_interest_rate.value() * notional_principal.value() * time_from_last_event);
        states.fee_accrued.add_assign(fee_rate.value() * notional_principal.value() * time_from_last_event);
        states.notional_principal.sub_assign(cbv.unwrap() * notional_principal.value());
        states.status_date = StatusDate::new(time.value()).ok();
        
    }
}
