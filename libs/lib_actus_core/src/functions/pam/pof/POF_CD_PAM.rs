use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::attributes::ContractTerms::ContractTerms;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::types::IsoDatetime::IsoDatetime;

use crate::attributes::ContractReference::ContractReference;
#[allow(non_camel_case_types)]
pub struct POF_CD_PAM;

impl TraitPayOffFunction for POF_CD_PAM {
    fn eval(
        &self,
        _time: &IsoDatetime,
        _states: &StatesSpace,
        _contract_terms: &ContractTerms,
        _contract_structure: &Option<Vec<ContractReference>>,
        _risk_factor_model: &Option<RiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        0.0
    }
}
