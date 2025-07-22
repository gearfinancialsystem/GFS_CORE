use lib_actus_terms::ContractTerms::ContractTerms;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_states_space::states_space::StatesSpace::StatesSpace;


use lib_actus_events::traits::TraitPayOffFunction::TraitPayOffFunction;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_types::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_DV_STK;


impl TraitPayOffFunction for POF_DV_STK {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StatesSpace,
        model: &ContractTerms,
        risk_factor_model: Option<&dyn TraitRiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let contract_role = model.contract_role.as_ref().expect("contract role should always be some");
        let quantity = model.quantity.clone().expect("quantity should always be some");
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            model,
            time,
            states
        );
        settlement_currency_fx_rate * contract_role.role_sign() * quantity.value() * 1.0
        
    }
}
