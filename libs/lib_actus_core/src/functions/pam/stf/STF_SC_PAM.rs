use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_SC_PAM;

impl TraitStateTransitionFunction for STF_SC_PAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayConvention,
    ) { // ->StateSpace
        // let mut new_state = StateSpace::copy_state_space(states);
        // let mut new_states: StateSpace = StateSpace::copy_state_space(states);
        // Calculate time from the last event
        let timeFromLastEvent = day_counter.day_count_fraction(
            time_adjuster.shift_bd(&states.statusDate.unwrap()),
            time_adjuster.shift_bd(&time),
        );

        states.accruedInterest = match (states.accruedInterest, states.nominalInterestRate, states.notionalPrincipal) {
            (Some(a), Some(b), Some(c)) => Some(a + (b * c * timeFromLastEvent)),
            (accrued_interest, _, _) => accrued_interest,
        };



        // new_state.accruedInterest = Some(new_state.accruedInterest.unwrap() + (new_state.nominalInterestRate.unwrap() * new_state.notionalPrincipal.unwrap() * timeFromLastEvent));
        // new_state.accruedInterest = match (new_state.accruedInterest, new_state.nominalInterestRate, new_state.notionalPrincipal) {
        //     (Some(accrued), Some(rate), Some(principal)) => {
        //         Some(accrued + (rate * principal * timeFromLastEvent))
        //     }
        //     (accrued, _, _) => accrued, // garde l'ancienne valeur si le calcul n'est pas possible
        // };

        states.feeAccrued = match (states.feeAccrued, model.feeRate, states.notionalPrincipal) {
            (Some(a), Some(b), Some(c)) => Some(a + (b * c * timeFromLastEvent)),
            (fee_accrued, _, _) => fee_accrued,
        };

        // new_state.feeAccrued = match (new_state.feeAccrued, model.feeRate, new_state.notionalPrincipal) {
        //     (Some(fee), Some(rate), Some(principal)) => {
        //         Some(fee + (rate * principal * timeFromLastEvent))
        //     }
        //     (fee, _, _) => fee, // garde l'ancienne valeur de feeAccrued si le calcul n'est pas possible
        // };
        //

        // // new_state.feeAccrued = Some(new_state.feeAccrued.unwrap() + (model.feeRate.unwrap() * new_state.notionalPrincipal.unwrap() * timeFromLastEvent));
        let scalingMultiplier = 1.0; // a corriger !!!
        states.interestScalingMultiplier = match (states.interestScalingMultiplier, scalingMultiplier) {
            (Some(a), b) => {
                if a.to_string().contains("I") {
                    Some(b)
                }
                else {
                    None
                }
            },
            (a, _) => a,
        };

        states.notionalScalingMultiplier = match (states.notionalScalingMultiplier, scalingMultiplier) {
            (Some(a), b) => {
                if a.to_string().contains("N") {
                    Some(b)
                }
                else {
                    None
                }
            },
            (a, _) => a,
        };


        states.statusDate = Some(*time);
        //
        // new_state.interestScalingMultiplier = match (new_state.interestScalingMultiplier, scalingMultiplier) {
        //     (Some(iScale), scalingMultiplier) => {
        //         if iScale.to_string().contains("I") {
        //             Some(iScale + scalingMultiplier)
        //         } else {
        //             None
        //         }
        //     }
        //     (iScale, _) => iScale, // garde l'ancienne valeur de feeAccrued si le calcul n'est pas possible
        // };
        // new_state.notionalScalingMultiplier = match (new_state.notionalScalingMultiplier, scalingMultiplier) {
        //     (Some(iScale), scalingMultiplier) => {
        //         if iScale.to_string().contains("N") {
        //             Some(iScale + scalingMultiplier)
        //         } else {
        //             None
        //         }
        //     }
        //     (iScale, _) => iScale, // garde l'ancienne valeur de feeAccrued si le calcul n'est pas possible
        // };
        //
        // new_state.statusDate = match (new_state.statusDate) {
        //     (Some(stdate)) => {
        //         Some(stdate)
        //     }
        //     (stdate) => stdate, // garde l'ancienne valeur si le calcul n'est pas possible
        // };
        //
        // // Return a copy of the updated state space
        // new_state

    }
}
