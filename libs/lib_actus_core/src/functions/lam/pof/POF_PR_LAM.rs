use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::external::RiskFactorModel::RiskFactorModel;

#[allow(non_camel_case_types)]
pub struct POF_PR_LAM;

impl TraitPayOffFunction for POF_PR_LAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StatesSpace,
        contract_terms: &ContractTerms,
        contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<RiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            contract_terms,
            time,
            states
        );
        
        let redemption = states.next_principal_redemption_payment.clone().unwrap().value()
            - contract_terms.contract_role.clone().unwrap().role_sign()
            * (states.next_principal_redemption_payment.clone().unwrap().value().abs() - states.notional_principal.clone().unwrap().value().abs()).max(0.0);

        settlement_currency_fx_rate
            * contract_terms.contract_role.clone().unwrap().role_sign()
            * states.notional_principal.clone().unwrap().value()
            * redemption
    }
}
