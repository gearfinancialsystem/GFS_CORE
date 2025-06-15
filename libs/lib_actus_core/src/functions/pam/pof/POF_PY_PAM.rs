use crate::terms::grp_optionality::PenaltyType::PenaltyType;
use crate::contracts::ContractModel::ContractModel;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::states::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::PayOffFunctionTrait::PayOffFunctionTrait;

#[allow(non_camel_case_types)]
pub struct POF_PY_PAM;

impl PayOffFunctionTrait for POF_PY_PAM {
    fn eval(
        &self,
        time: IsoDatetime, 
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayConvention,
    ) -> f64 {
        match &*model.PenaltyType {
            PenaltyType::A(A) => {
                    1.0           // implémenter settlement_currency_fx_rate dans common util
                    * model.ContractRole.role_sign()
                    * model.PenaltyRate.unwrap()
            }
            PenaltyType::N(N) => {
                    1.0           // implémenter settlement_currency_fx_rate dans common util
                    * model.ContractRole.role_sign()
                    * day_counter.day_count_fraction(
                        time_adjuster.shift_bd(&states.statusDate),
                        time_adjuster.shift_bd(&time),
                    ) * model.PenaltyRate.unwrap() * states.notionalPrincipal.as_deref().unwrap()
            }
            _ => {
                    1.0           // implémenter settlement_currency_fx_rate dans common util
                    * model.ContractRole.role_sign()
                    * day_counter.day_count_fraction(
                        time_adjuster.shift_bd(&states.statusDate),
                        time_adjuster.shift_bd(&time),
                    ) * states.notionalPrincipal.as_deref().unwrap()
                    * (0.0_f64.max(states.nominalInterestRate - 
                        1.0)) // risk_factor_model.state_at(model.MarketObjectCodeOfRateReset, &time, states, model) remettre a la place de 1.0
            }
        }
    }
}
