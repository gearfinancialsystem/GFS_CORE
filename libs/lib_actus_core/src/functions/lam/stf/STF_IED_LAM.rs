use std::cmp::PartialEq;
use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_interest::interest_calculation_base::Nt::NT;
use crate::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;


#[allow(non_camel_case_types)]
pub struct STF_IED_LAM;

impl TraitStateTransitionFunction for STF_IED_LAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let contract_role = model.contractRole.as_ref().expect("contractRole should always be Some");
        let notional_principal = model.notionalPrincipal.expect("notionalPrincipal should always be Some");
        let nominal_interest_rate = model.nominalInterestRate.expect("nominalInterestRate should always be Some");

        states.statusDate = Some(*time);
        states.notionalPrincipal = Some(contract_role.role_sign() * notional_principal);
        states.nominalInterestRate = Some(nominal_interest_rate);

        if let Some(interest_calculation_base) = &model.interestCalculationBase {
            if *interest_calculation_base == InterestCalculationBase::NT(NT) {
                states.interestCalculationBaseAmount = states.notionalPrincipal;
            } else {
                let interest_calculation_base_amount = model.interestCalculationBaseAmount.expect("interestCalculationBaseAmount should always be Some");
                states.interestCalculationBaseAmount = Some(contract_role.role_sign() * interest_calculation_base_amount);
            }
        }

        if let Some(accrued_interest) = model.accruedInterest {
            states.accruedInterest = Some(contract_role.role_sign() * accrued_interest);
        } else if let Some(cycle_anchor_date_of_interest_payment) = model.cycleAnchorDateOfInterestPayment {
            if cycle_anchor_date_of_interest_payment < *time {
                let time_from_last_event = day_counter.day_count_fraction(
                    time_adjuster.shift_sc(&cycle_anchor_date_of_interest_payment),
                    time_adjuster.shift_sc(time)
                );
                states.accruedInterest = states.notionalPrincipal.map(|np| {
                    np * states.interestCalculationBaseAmount.unwrap_or(0.0) * time_from_last_event
                });
            } else {
                states.accruedInterest = Some(0.0);
            }
        } else {
            states.accruedInterest = Some(0.0);
        }
    }
}
