use std::cmp::max;
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
        
        let penalty_type = model.penaltyType.as_ref().expect("penaltyType should be Some");
        let contract_role = model.contractRole.as_ref().expect("contract role should be Some");

        match penalty_type {
            PenaltyType::A(_A) => {
                let penalty_rate = model.penaltyRate.as_ref().expect("penaltyRate should be Some");
                1.0 * contract_role.role_sign() * penalty_rate
            }
            PenaltyType::N(_N) => {
                let penalty_rate = model.penaltyRate.as_ref().expect("penaltyRate should be Some");
                let status_date = states.statusDate.as_ref().expect("status date should always exist");
                let notional_principal = states.notionalPrincipal.as_ref().expect("notionalPrincipal should be Some");

                1.0 * contract_role.role_sign()
                    * day_counter.day_count_fraction(time_adjuster.shift_bd(&status_date), time_adjuster.shift_bd(&time))
                * penalty_rate * notional_principal
            }
            _ => {
                let status_date = states.statusDate.expect("status date should always exist");
                let notional_principal = states.notionalPrincipal.expect("notionalPrincipal should always exist");
                let nominal_interest_rate = states.nominalInterestRate.expect("nominalInterestRate should be Some");
                let market_object_code_of_rate_reset = model.marketObjectCodeOfRateReset.as_ref().expect("marketObjectCodeOfRateReset should be Some");
                
                1.0 * contract_role.role_sign()
                    * day_counter.day_count_fraction(time_adjuster.shift_bd(&status_date), time_adjuster.shift_bd(&time))
                    * notional_principal
                    * 0.0f64.max(nominal_interest_rate - 1.0f64)
            }
        }
    }
}
