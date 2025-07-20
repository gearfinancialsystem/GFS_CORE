use crate::attributes::ContractTerms::ContractTerms;

use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;
#[allow(non_camel_case_types)]
pub struct POF_TD_PAM;

impl TraitPayOffFunction for POF_TD_PAM {
    fn eval(
        &self,
        time: &IsoDatetime, 
        states: &StateSpace,
        model: &ContractTerms,
        risk_factor_model: &DataObserver,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let contract_role = model.contract_role.as_ref().expect("contract role should always be some");
        let price_at_termination_date = model.price_at_termination_date.as_ref().expect("priceAtTerminationDate should always exist");
        let accrued_interest = states.accrued_interest.as_ref().expect("accruedInterest should always exist");
        let status_date = states.status_date.as_ref().expect("status date should always exist");
        let nominal_interest_rate = states.nominal_interest_rate.as_ref().expect("nominalInterestRate should be Some");
        let notional_principal = states.notional_principal.as_ref().expect("notionalPrincipal should always exist");
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            model,
            time,
            states
        );
        
        settlement_currency_fx_rate *
            contract_role.role_sign() *
            (price_at_termination_date.value() +
            accrued_interest.value() + 
            day_counter.day_count_fraction(
                time_adjuster.shift_sc(&status_date.value()),
                time_adjuster.shift_sc(time)
            ) * nominal_interest_rate.value()
                * notional_principal.value()
            )
        
    }
}
