use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::external::RiskFactorModel::RiskFactorModel;

#[allow(non_camel_case_types)]
pub struct POF_TD_FXOUT;

impl TraitPayOffFunction for POF_TD_FXOUT {
    fn eval(
        &self,
        _time: &PhantomIsoDatetimeW,
        _states: &StatesSpace,
        contract_terms: &ContractTerms,
contract_structure: &Option<Vec<ContractReference>>,
        _risk_factor_model: &Option<impl TraitRiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let price_at_termination_date = contract_terms.price_at_termination_date.clone().expect("priceAtTerminationDate should always exist");

        1.0 * price_at_termination_date.value()
    }
}
