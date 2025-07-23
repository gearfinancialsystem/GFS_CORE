use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;


use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::traits::TraitOptionExt::TraitOptionExt;
use crate::types::IsoDatetime::IsoDatetime;


use crate::external::RiskFactorModel::RiskFactorModel;
use crate::attributes::ContractReference::ContractReference;

#[allow(non_camel_case_types)]
pub struct STF_PP_PAM;

impl TraitStateTransitionFunction for STF_PP_PAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        contract_terms: &ContractTerms,
        _contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<RiskFactorModel>,
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
            time_adjuster.shift_sc(&status_date.value()),
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
        states.status_date = Some(StatusDate::from(*time));
        
    }
}
