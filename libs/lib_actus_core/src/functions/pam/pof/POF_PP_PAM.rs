use lib_actus_events::traits::TraitPayOffFunction::TraitPayOffFunction;
use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;

use lib_actus_types::types::IsoDatetime::IsoDatetime;

use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;

#[allow(non_camel_case_types)]
pub struct POF_PP_PAM;

impl TraitPayOffFunction for POF_PP_PAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StatesSpace,
        model: &ContractTerms,
        risk_factor_model: Option<&dyn TraitRiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        
            let notional_principal = states.notional_principal.as_ref().expect("notionalPrincipal should always be some");
            let contract_role = model.contract_role.as_ref().expect("contract role should always be some");

            let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
                risk_factor_model,
                model,
                time,
                states
            );
            settlement_currency_fx_rate * contract_role.role_sign() * 1.0 * notional_principal.value()

    }
}
