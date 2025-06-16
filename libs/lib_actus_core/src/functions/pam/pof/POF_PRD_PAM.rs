use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_PRD_PAM;

impl TraitPayOffFunction for POF_PRD_PAM {
    fn eval(
        &self,
        time: &IsoDatetime, 
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayConvention,
    ) -> f64 {
            match (&model.contractRole, &model.priceAtPurchaseDate, states.accruedInterest, states.nominalInterestRate, states.notionalPrincipal) {
                (Some(a), Some(b), Some(c), Some(d), Some(e)) => 1.0 * -1.0 * a.role_sign() * (b + c + day_counter.day_count_fraction(
                    time_adjuster.shift_bd(&states.statusDate.unwrap()),
                    time_adjuster.shift_bd(&time)
                ) * d * e),
                (a, _, _, _, _) => 1.0, // a verifier
            }
    }
}
