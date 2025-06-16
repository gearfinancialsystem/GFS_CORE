use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_fees::FeeBasis::FeeBasis;
use crate::terms::grp_fees::fee_basis::N::N;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_IP_PAM;

impl TraitStateTransitionFunction for STF_IP_PAM {
    fn eval(
        &self,
        time: &IsoDatetime, 
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayConvention,
    ) {
        // Reset accrued interest
        //let mut new_states: StateSpace = states.copy_state_space(); 

        states.accruedInterest = Some(0.0);

        // Update fee-accrued

        if let Some(fee_basis) = &model.feeBasis {
            if *fee_basis == FeeBasis::N(N) {
                let time_fraction = day_counter.day_count_fraction(
                    time_adjuster.shift_bd(&states.statusDate.unwrap()),
                    time_adjuster.shift_bd(&time),
                );


                states.feeAccrued = match (states.feeAccrued, model.feeRate, states.notionalPrincipal) {
                    (Some(a), Some(b), Some(c)) => Some(a + (b * c)),
                    (feeAccrued, _, _) => feeAccrued,
                };
            }
            // Further processing for FeeBasis cases that aren't "N"
            // Commented out: If additional logic is needed, it can be expanded here

            // Update the status date
            states.statusDate = Some(*time);
        }
    }
}
