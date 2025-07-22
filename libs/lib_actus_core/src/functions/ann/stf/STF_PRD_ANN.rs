use crate::attributes::ContractTerms::ContractModel;

use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_notional_principal::NextPrincipalRedemptionPayment::NextPrincipalRedemptionPayment;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::traits::TraitOptionExt::TraitOptionExt;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;

#[allow(non_camel_case_types)]
pub struct STF_PRD_ANN;
impl TraitStateTransitionFunction for STF_PRD_ANN {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &DataObserver,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    )  {

        let status_date = states.status_date.clone().expect("status date should be some");
        let nominal_interest_rate = states.nominal_interest_rate.clone().expect("nominalInterestRate should be some");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.clone().expect("nominalInterestRate should be some");
        let notional_principal = states.notional_principal.clone().expect("notionalPrincipal should be some");

        let fee_rate_m = model.clone().fee_rate.clone().expect("feeRate should be some");
        let contract_role_m = model.clone().contract_role.clone().expect("contract role should be some");
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
