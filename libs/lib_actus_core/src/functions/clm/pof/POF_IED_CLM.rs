use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_events::traits::TraitPayOffFunction::TraitPayOffFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::util::CommonUtils::CommonUtils as cu;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;

#[allow(non_camel_case_types)]
pub struct POF_IED_CLM;

impl TraitPayOffFunction for POF_IED_CLM {
    fn eval(
        &self,
        time: &IsoDatetime, 
        states: &StatesSpace,
        model: &ContractTerms,
        risk_factor_model: Option<&dyn TraitRiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        
        let contract_role = model.contract_role.clone().expect("contract role should always exist");
        let notional_principal = model.notional_principal.clone().expect("notionalPrincipal should always exist");
        let settlement_currency_fx_rate = cu::settlementCurrencyFxRate(
            risk_factor_model,
            model,
            time,
            states
        );

        settlement_currency_fx_rate
            * contract_role.role_sign()
            * (-1.0)
            * notional_principal.value()
    }
}
