use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_IPCI_LAM;

impl TraitStateTransitionFunction for STF_IPCI_LAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let status_date = states.status_date.expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.expect("nominalInterestRate should always be Some");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.expect("interestCalculationBaseAmount should always be Some");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date),
            time_adjuster.shift_sc(time)
        );

        states.notional_principal = states.notional_principal.map(|notional_principal| {
            notional_principal + states.accrued_interest.unwrap_or(0.0) + (nominal_interest_rate * interest_calculation_base_amount * time_from_last_event)
        });

        states.accrued_interest = Some(0.0);

        states.fee_accrued = states.fee_accrued.map(|fee_accrued| {
            let fee_rate = model.fee_rate.unwrap_or(0.0);
            fee_accrued + fee_rate * states.notional_principal.unwrap_or(0.0) * time_from_last_event
        });

        states.status_date = Some(*time);
    }
}
