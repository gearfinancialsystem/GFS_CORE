use crate::attributes::ContractTerms::ContractModel;

use crate::state_space::StateSpace::StateSpace;
use lib_actus_events::traits::TraitPayOffFunction::TraitPayOffFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::util_tests::essai_data_observer::DataObserver;

#[allow(non_camel_case_types)]
pub struct POF_PR_LAM;

impl TraitPayOffFunction for POF_PR_LAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StatesSpace,
        model: &ContractModel,
        risk_factor_model: &DataObserver,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            model,
            time,
            states
        );
        
        let redemption = states.next_principal_redemption_payment.clone().unwrap().value()
            - model.contract_role.clone().unwrap().role_sign()
            * (states.next_principal_redemption_payment.clone().unwrap().value().abs() - states.notional_principal.clone().unwrap().value().abs()).max(0.0);

        settlement_currency_fx_rate
            * model.contract_role.clone().unwrap().role_sign()
            * states.notional_principal.clone().unwrap().value()
            * redemption
    }
}
