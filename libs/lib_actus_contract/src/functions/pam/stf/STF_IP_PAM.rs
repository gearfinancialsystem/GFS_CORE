use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::attributes::ContractTerms::ContractTerms;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use lib_actus_terms::traits::TraitOptionExt::TraitOptionExt;

use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::attributes::ContractReference::ContractReference;
#[allow(non_camel_case_types)]
pub struct STF_IP_PAM;

impl TraitStateTransitionFunction for STF_IP_PAM {
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
        let day_counter = day_counter.clone().expect("sould have day counter");
        let status_date = states.status_date.as_ref().expect("status date should be some");
        let fee_rate_m = contract_terms.fee_rate.clone().expect("fee rate should be some");
        let notional_principal = states.notional_principal.clone().expect("notional principal should be some");


        let time_from_last_event = day_counter.day_count_fraction(time_adjuster.shift_sc(&status_date.value()), time_adjuster.shift_sc(time));

        states.accrued_interest = AccruedInterest::new(0.0).ok();

        states.fee_accrued.add_assign(fee_rate_m.value() * notional_principal.value() * time_from_last_event);

        states.status_date = Some(StatusDate::from(*time));

    }
}
