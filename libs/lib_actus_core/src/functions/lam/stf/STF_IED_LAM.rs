use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_interest::interest_calculation_base::Nt::NT;
use crate::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
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
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let contract_role = model.contract_role.as_ref().expect("contractRole should always be Some");
        let notional_principal = model.notional_principal.expect("notionalPrincipal should always be Some");
        let nominal_interest_rate = model.nominal_interest_rate.expect("nominalInterestRate should always be Some");

        states.status_date = Some(*time);
        states.notional_principal = Some(contract_role.role_sign() * notional_principal);
        states.nominal_interest_rate = Some(nominal_interest_rate);

        if let Some(interest_calculation_base) = &model.interest_calculation_base {
            if *interest_calculation_base == InterestCalculationBase::NT(NT) {
                states.interest_calculation_base_amount = states.notional_principal;
            } else {
                let interest_calculation_base_amount = model.interest_calculation_baseAmount.expect("interestCalculationBaseAmount should always be Some");
                states.interest_calculation_base_amount = Some(contract_role.role_sign() * interest_calculation_base_amount);
            }
        }

        if let Some(accrued_interest) = model.accruedInterest {
            states.accrued_interest = Some(contract_role.role_sign() * accrued_interest);
        } else if let Some(cycle_anchor_date_of_interest_payment) = model.cycleAnchorDateOfInterestPayment {
            if cycle_anchor_date_of_interest_payment < *time {
                let time_from_last_event = day_counter.day_count_fraction(
                    time_adjuster.shift_sc(&cycle_anchor_date_of_interest_payment),
                    time_adjuster.shift_sc(time)
                );
                states.accrued_interest = states.notional_principal.map(|np| {
                    np * states.interest_calculation_base_amount.unwrap_or(0.0) * time_from_last_event
                });
            } else {
                states.accrued_interest = Some(0.0);
            }
        } else {
            states.accrued_interest = Some(0.0);
        }
    }
}
