use crate::attributes::ContractReference::ContractReference;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_notional_principal::NextPrincipalRedemptionPayment::NextPrincipalRedemptionPayment;
use crate::traits::TraitOptionExt::TraitOptionExt;
use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use crate::types::IsoDatetime::IsoDatetime;


#[allow(non_camel_case_types)]
pub struct STF_PRD_ANN;
impl TraitStateTransitionFunction for STF_PRD_ANN {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        contract_terms: &ContractTerms,
        contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<RiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    )  {

        let status_date = states.status_date.clone().expect("status date should be some");
        let nominal_interest_rate = states.nominal_interest_rate.clone().expect("nominalInterestRate should be some");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.clone().expect("nominalInterestRate should be some");
        let notional_principal = states.notional_principal.clone().expect("notionalPrincipal should be some");

        let fee_rate_m = contract_terms.clone().fee_rate.clone().expect("feeRate should be some");
        let contract_role_m = contract_terms.clone().contract_role.clone().expect("contract role should be some");
        let day_counter = day_counter.clone().expect("sould have day counter");
        
        let time_from_last_event = day_counter.day_count_fraction(time_adjuster.shift_sc(&status_date.value()),
                                                                  time_adjuster.shift_sc(time));

        states.accrued_interest.add_assign(time_from_last_event *
            nominal_interest_rate.value() * interest_calculation_base_amount.value());

        states.fee_accrued.add_assign(time_from_last_event * notional_principal.value() * fee_rate_m.value());
        

        states.status_date = Some(StatusDate::from(*time));
        states.next_principal_redemption_payment = NextPrincipalRedemptionPayment::new(
            contract_role_m.role_sign() * 1.0).ok(); // implementer redemptionm utile

    }
}
