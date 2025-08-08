use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use crate::attributes::ContractTerms::ContractTerms;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::states_space::StatesSpace::StatesSpace;


use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::types::IsoDatetime::IsoDatetime;
use crate::attributes::ContractReference::ContractReference;
#[allow(non_camel_case_types)]
pub struct POF_DV_STK;


impl TraitPayOffFunction for POF_DV_STK {
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &StatesSpace,
        contract_terms: &ContractTerms,
        _contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<impl TraitRiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let contract_role = contract_terms.contract_role.as_ref().expect("contract role should always be some");
        let quantity = contract_terms.quantity.clone().expect("quantity should always be some");
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            contract_terms,
            time,
            states
        );

        let mut cbv = None;
        if let Some(rfm) = risk_factor_model {
            cbv = rfm.state_at(
                contract_terms.market_object_code_of_dividends.clone().unwrap().value(),
                time,
                states,
                contract_terms,
                true
            );
        } else {
            cbv = None
        }

        settlement_currency_fx_rate * contract_role.role_sign() * quantity.value() * cbv.unwrap()
        
    }
}
