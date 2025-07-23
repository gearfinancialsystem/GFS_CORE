use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_fees::FeeBasis::FeeBasis;
use lib_actus_terms::terms::grp_fees::fee_basis::A::A;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;

use lib_actus_events::traits::TraitPayOffFunction::TraitPayOffFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_types::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_FP_CEG;

impl TraitPayOffFunction for POF_FP_CEG {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StatesSpace,
        model: &ContractTerms,
        risk_factor_model: Option<&dyn TraitRiskFactorModel>,
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
