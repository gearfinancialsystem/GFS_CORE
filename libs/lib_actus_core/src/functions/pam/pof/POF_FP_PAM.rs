use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_fees::fee_basis::A::A;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::isoDatetime::IsoDatetime;
use crate::terms::grp_fees::FeeBasis::FeeBasis;
#[allow(non_camel_case_types)]
pub struct POF_FP_PAM;

impl TraitPayOffFunction for POF_FP_PAM {
    fn eval(
        &self,
        _time: &IsoDatetime, 
        states: &StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayConvention,
    ) -> f64 {
        match &model.feeBasis {
            Some(fee_basis) => {
                if *fee_basis == FeeBasis::A(A) {
                    let role_sign = model.contractRole.as_ref()
                        .map_or(0.0, |role| role.role_sign());
                    let fee_rate = model.feeRate.unwrap_or(0.0);
                    1.0 * role_sign * fee_rate
                } else {
                    let fee_accrued = states.feeAccrued.unwrap_or(0.0);
                    let fee_rate = model.feeRate.unwrap_or(0.0);
                    let notional_principal = states.notionalPrincipal.unwrap_or(0.0);
                    1.0 * fee_accrued * fee_rate * notional_principal
                }
            }
            None => 0.0,
        }
    }

}

