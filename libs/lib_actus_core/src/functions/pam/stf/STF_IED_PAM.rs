use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_IED_PAM;

impl TraitStateTransitionFunction for STF_IED_PAM {
    fn eval(
        &self,
        time: &IsoDatetime, 
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    )  {
        
        let contract_role = model.contract_role.as_ref().expect("contract role should be Some");
        let notional_principal = model.notional_principal.expect("notionalPrincipal should always be Some");
        let nominal_interest_rate = model.nominal_interest_rate.expect("nominalInterestRate should be Some");
        let notional_principal_s = states.notional_principal.expect("notionalPrincipal should always be Some");
        let nominal_interest_rate_s = states.nominal_interest_rate.expect("nominalInterestRate should be Some");

        states.notional_principal = Some(contract_role.role_sign() * notional_principal);
        states.nominal_interest_rate = Some(nominal_interest_rate);
        states.status_date = Some(*time);

        if let (Some(cycle_anchor_date), Some(initial_exchange_date)) = (
            model.cycleAnchorDateOfInterestPayment.as_ref(),
            model.initial_exchange_date.as_ref(),
        ) {
            if cycle_anchor_date < initial_exchange_date {
                states.accrued_interest = states.accrued_interest.map(|mut accrued_interest| {
                    accrued_interest += notional_principal_s * nominal_interest_rate_s *
                        day_counter.day_count_fraction(
                            time_adjuster.shift_sc(cycle_anchor_date),
                            time_adjuster.shift_sc(time)
                        );
                    accrued_interest
                });
            }
        }
        
        
    }
}
