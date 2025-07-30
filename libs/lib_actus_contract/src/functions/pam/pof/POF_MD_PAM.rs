use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;

use lib_actus_types::types::IsoDatetime::IsoDatetime;

use crate::external::RiskFactorModel::RiskFactorModel;
use crate::attributes::ContractReference::ContractReference;
#[allow(non_camel_case_types)]
pub struct POF_MD_PAM;

impl TraitPayOffFunction for POF_MD_PAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StatesSpace,
        _contract_terms: &ContractTerms,
        _contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<RiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        
            let notional_scaling_multiplier = states.notional_scaling_multiplier.as_ref().expect("notionalScalingMultiplier should always be some");
            let notional_principal = states.notional_principal.as_ref().expect("notionalPrincipal should always be some");

            let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
                risk_factor_model,
                _contract_terms,
                time,
                states
            );
            settlement_currency_fx_rate * notional_scaling_multiplier.value() * notional_principal.value()
        
    }
}
