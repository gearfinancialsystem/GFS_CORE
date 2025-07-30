use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
// use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_types::types::IsoDatetime::IsoDatetime;

use crate::attributes::ContractReference::ContractReference;
#[allow(non_camel_case_types)]
pub struct POF_AD_PAM;


impl TraitPayOffFunction for POF_AD_PAM {
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
