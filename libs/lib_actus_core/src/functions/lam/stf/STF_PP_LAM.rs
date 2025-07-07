use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_interest::interest_calculation_base::Ntl::NTL;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;

#[allow(non_camel_case_types)]
pub struct STF_PP_LAM;

impl TraitStateTransitionFunction for STF_PP_LAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let status_date = states.status_date.expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.expect("nominalInterestRate should always be Some");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.expect("interestCalculationBaseAmount should always be Some");
        let notional_principal = states.notional_principal.expect("notionalPrincipal should always be Some");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date),
            time_adjuster.shift_sc(time)
        );

        states.accrued_interest = states.accrued_interest.map(|accrued_interest| {
            accrued_interest + nominal_interest_rate * interest_calculation_base_amount * time_from_last_event
        });

        states.fee_accrued = states.fee_accrued.map(|fee_accrued| {
            let fee_rate = model.fee_rate.unwrap_or(0.0);
            fee_accrued + fee_rate * notional_principal * time_from_last_event
        });

        // let prepayment_factor = risk_factor_model.state_at(
        //     &model.objectCodeOfPrepaymentModel,
        //     time,
        //     states,
        //     model,
        //     false
        // );
        let prepayment_factor = 1.0;

        states.notional_principal = Some(notional_principal - prepayment_factor * notional_principal);

        if let Some(interest_calculation_base) = &model.interest_calculation_base {
            if *interest_calculation_base == InterestCalculationBase::NTL(NTL) {
                states.interest_calculation_base_amount = states.notional_principal;
            }
        }

        states.status_date = Some(*time);
    }
}
