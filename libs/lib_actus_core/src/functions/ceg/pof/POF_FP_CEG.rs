use crate::attributes::ContractModel::ContractModel;

use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_fees::FeeBasis::FeeBasis;
use crate::terms::grp_fees::fee_basis::A::A;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;

#[allow(non_camel_case_types)]
pub struct POF_FP_CEG;

impl TraitPayOffFunction for POF_FP_CEG {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &DataObserver,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            model,
            time,
            states
        );
        let contract_role = model.contract_role.as_ref().expect("contract role should always exist");
        let fee_rate = model.fee_rate.clone().expect("feeRate should always exist");

        let day_counter = day_counter.clone().expect("sould have day counter");
        
        let payoff = if FeeBasis::A(A) == model.fee_basis.clone().unwrap() {
            settlement_currency_fx_rate * contract_role.role_sign() * fee_rate.value()
        } else {
            let time_from_last_event = day_counter.day_count_fraction(
                time_adjuster.shift_sc(&states.status_date.clone().unwrap().value()),
                time_adjuster.shift_sc(time)
            );
            settlement_currency_fx_rate * (
                states.fee_accrued.clone().unwrap().value() +
                    (states.notional_principal.clone().unwrap().value() * time_from_last_event * fee_rate.value())
            )
        };

        payoff
    }
}
