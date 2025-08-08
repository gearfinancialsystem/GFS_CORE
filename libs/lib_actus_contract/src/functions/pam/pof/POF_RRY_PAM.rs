use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;

use lib_actus_types::types::IsoDatetime::IsoDatetime;

use crate::attributes::ContractReference::ContractReference;
use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
#[allow(non_camel_case_types)]
pub struct POF_RRY_PAM;

impl TraitPayOffFunction for POF_RRY_PAM {
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &StatesSpace,
        contract_terms: &ContractTerms,
contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<impl TraitRiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        0.0
    }
}
