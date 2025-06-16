use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_optionality::PenaltyType::PenaltyType;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_PY_PAM;

impl TraitPayOffFunction for POF_PY_PAM {
    fn eval(
        &self,
        time: &IsoDatetime, 
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayConvention,
    ) -> f64 {

        match &model.penaltyType {
            Some(PenaltyType::A(A)) => {
                match (&model.contractRole, &model.penaltyRate) {
                    (Some(a), Some(b)) => 1.0 * a.role_sign() * b,
                    (a, _) => 1.0,
                }
            }
            Some(PenaltyType::N(N)) => {
                match (&model.contractRole, &model.penaltyRate, states.notionalPrincipal) {
                    (Some(a), Some(b), Some(c)) => 1.0 * a.role_sign() * day_counter.day_count_fraction(
                        time_adjuster.shift_bd(&states.statusDate.unwrap()), time_adjuster.shift_bd(&time)) * b * c,
                    (a, _, _) => 1.0,
                }
            }

            _ => {
                    match (&model.contractRole, states.notionalPrincipal, states.nominalInterestRate) {
                        (Some(a), Some(b), Some(c)) => 1.0 * a.role_sign() * day_counter.day_count_fraction(
                            time_adjuster.shift_bd(&states.statusDate.unwrap()), time_adjuster.shift_bd(&time)) * b * c,
                        (a, _, _) => 1.0,
                    } // a reecrire par rapport a la source
            }
        }
    }
}
