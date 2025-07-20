

use crate::attributes::ContractTerms::ContractTerms;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;
#[allow(non_camel_case_types)]
pub struct POF_IED_PAM;


impl TraitPayOffFunction for POF_IED_PAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StateSpace,
        model: &ContractTerms,
        risk_factor_model: &DataObserver,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        
        let contract_role = model.contract_role.as_ref().expect("contract role should always be Some");
        let notional_principal = model.notional_principal.as_ref().expect("notionalPrincipal should always be Some");
        let premium_discount = model.premium_discount_at_ied.as_ref().expect("premiumDiscount should always be Some");
        
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            model,
            time,
            states
        );
        settlement_currency_fx_rate * contract_role.role_sign() * -1.0 * (notional_principal.value() + premium_discount.value())

    }
}
