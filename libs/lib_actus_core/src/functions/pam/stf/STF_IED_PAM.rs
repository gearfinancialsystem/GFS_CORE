use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;

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
        
        let contract_role = model.contractRole.as_ref().expect("contract role should be Some");
        let notional_principal = model.notionalPrincipal.expect("notionalPrincipal should always be Some");
        let nominal_interest_rate = model.nominalInterestRate.expect("nominalInterestRate should be Some");
        let notional_principal_s = states.notionalPrincipal.expect("notionalPrincipal should always be Some");
        let nominal_interest_rate_s = states.nominalInterestRate.expect("nominalInterestRate should be Some");

        states.notionalPrincipal = Some(contract_role.role_sign() * notional_principal);
        states.nominalInterestRate = Some(nominal_interest_rate);
        states.statusDate = Some(*time);

        if let (Some(cycle_anchor_date), Some(initial_exchange_date)) = (
            model.cycleAnchorDateOfInterestPayment.as_ref(),
            model.initialExchangeDate.as_ref(),
        ) {
            if cycle_anchor_date < initial_exchange_date {
                states.accruedInterest = states.accruedInterest.map(|mut accrued_interest| {
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
