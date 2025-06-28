use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_fees::FeeBasis::FeeBasis;
use crate::terms::grp_fees::fee_basis::A::A;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::isoDatetime::IsoDatetime;


#[allow(non_camel_case_types)]
pub struct POF_FP_CEG;

impl TraitPayOffFunction for POF_FP_CEG {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            model,
            time,
            states
        );
        let contract_role = model.contractRole.as_ref().expect("contract role should always exist");
        let fee_rate = model.feeRate.expect("feeRate should always exist");

        let payoff = if FeeBasis::A(A) == model.feeBasis.clone().unwrap() {
            settlement_currency_fx_rate * contract_role.role_sign() * fee_rate
        } else {
            let time_from_last_event = day_counter.day_count_fraction(
                time_adjuster.shift_sc(&states.statusDate.clone().unwrap()),
                time_adjuster.shift_sc(time)
            );
            settlement_currency_fx_rate * (
                states.feeAccrued.clone().unwrap() +
                    (states.notionalPrincipal.clone().unwrap() * time_from_last_event * fee_rate)
            )
        };

        payoff
    }
}
