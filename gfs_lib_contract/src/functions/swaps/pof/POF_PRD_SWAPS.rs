use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::attributes::ContractTerms::{ContractTerms};
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::attributes::ContractReference::ContractReference;
use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;

#[allow(non_camel_case_types)]
pub struct POF_PRD_SWAPS;

impl TraitPayOffFunction for POF_PRD_SWAPS {
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &StatesSpace,
        contract_terms: &ContractTerms,
contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<impl TraitRiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let price_at_purchase_date = contract_terms.price_at_purchase_date.clone().expect("Price at purchase date should always be Some");
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            contract_terms,
            time,
            states
        );
        
        settlement_currency_fx_rate * price_at_purchase_date.value()
    }
}