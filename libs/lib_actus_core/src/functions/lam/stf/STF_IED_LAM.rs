use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_interest::interest_calculation_base::Nt::NT;
use crate::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use crate::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;


#[allow(non_camel_case_types)]
pub struct STF_IED_LAM;

impl TraitStateTransitionFunction for STF_IED_LAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let contract_role = model.contract_role.clone().expect("contractRole should always be Some");
        let notional_principal = model.notional_principal.clone().expect("notionalPrincipal should always be Some");
        let nominal_interest_rate = model.nominal_interest_rate.clone().expect("nominalInterestRate should always be Some");

        states.status_date = Some(StatusDate::from(*time));
        states.notional_principal = NotionalPrincipal::new(contract_role.role_sign() * notional_principal.value()).ok();
        states.nominal_interest_rate = Some(nominal_interest_rate);

        if let Some(interest_calculation_base) = &model.interest_calculation_base {
            if *interest_calculation_base == InterestCalculationBase::NT(NT) {
                states.interest_calculation_base_amount = InterestCalculationBaseAmount::new(states.notional_principal.clone().unwrap().value()).ok() ;
            } else {
                let interest_calculation_base_amount = model.interest_calculation_base_amount.clone().expect("interestCalculationBaseAmount should always be Some");
                states.interest_calculation_base_amount = InterestCalculationBaseAmount::new(contract_role.role_sign() * interest_calculation_base_amount.value()).ok();
            }
        }

        if let Some(accrued_interest) = model.accrued_interest.clone() {
            states.accrued_interest = AccruedInterest::new(contract_role.role_sign() * accrued_interest.value()).ok();
        } else if let Some(cycle_anchor_date_of_interest_payment) = model.cycle_anchor_date_of_interest_payment.clone() {
            if cycle_anchor_date_of_interest_payment.value() < *time {
                let time_from_last_event = day_counter.day_count_fraction(
                    time_adjuster.shift_sc(&cycle_anchor_date_of_interest_payment.value()),
                    time_adjuster.shift_sc(time)
                );

                states.accrued_interest = AccruedInterest::new({
                    states.notional_principal.clone().unwrap().value() *
                        {
                            if states.interest_calculation_base_amount.is_none() {
                                0.0
                            }
                            else {
                                states.interest_calculation_base_amount.clone().unwrap().value()
                            }
                        } * time_from_last_event
                }).ok();

            } else {
                states.accrued_interest = AccruedInterest::new(0.0).ok();
            }
        } else {
            states.accrued_interest = AccruedInterest::new(0.0).ok();
        }
    }
}
