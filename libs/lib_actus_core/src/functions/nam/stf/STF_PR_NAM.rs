use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_fees::FeeRate::FeeRate;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;


#[allow(non_camel_case_types)]
pub struct STF_PR_NAM;

impl TraitStateTransitionFunction for STF_PR_NAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let status_date = states.status_date.clone().expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.clone().expect("nominalInterestRate should always be Some");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.clone().expect("interestCalculationBaseAmount should always be Some");
        let notional_principal = states.notional_principal.clone().expect("notionalPrincipal should always be Some");
        let next_principal_redemption_payment = states.next_principal_redemption_payment.clone().expect("nextPrincipalRedemptionPayment should always be Some");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.value()),
            time_adjuster.shift_sc(time)
        );

        states.accrued_interest = states.accrued_interest.clone().map(|mut accrued_interest| {
            accrued_interest += nominal_interest_rate.value() * interest_calculation_base_amount.value() * time_from_last_event;
            accrued_interest
        });

        states.fee_accrued = states.fee_accrued.clone().map(|mut fee_accrued| {
            let fee_rate = {
                if model.fee_rate.is_some() {
                    FeeRate::new(0.0).ok().unwrap()
                }
                else {
                    model.fee_rate.clone().unwrap()
                }
            };
            fee_accrued += fee_rate.value() * notional_principal.value() * time_from_last_event;
            fee_accrued
        });

        let contract_role = model.contract_role.clone().expect("contractRole should always be Some");
        let role_sign = contract_role.role_sign();
        let redemption_amount = next_principal_redemption_payment.value() - role_sign * {
            if states.accrued_interest.is_none() {
                AccruedInterest::new(0.0).ok().unwrap()
            }
            else {
                states.accrued_interest.clone().unwrap()
            }
        }.value();

        let redemption = redemption_amount - redemption_amount.max(0.0).min(notional_principal.value().abs());

        states.notional_principal = NotionalPrincipal::new({
            if states.notional_principal.is_none() {
                NotionalPrincipal::new(0.0).ok().unwrap()
            }
            else {
                states.notional_principal.clone().unwrap()
            }
        }.value() - role_sign * redemption).ok();

        states.status_date = Some(StatusDate::from(*time));
    }
}
