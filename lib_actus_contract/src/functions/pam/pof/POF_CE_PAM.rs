use crate::traits::TraitPayOffFunction::TraitPayOffFunction;

use crate::attributes::ContractTerms::ContractTerms;
use crate::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;

// use crate::attributes::ContractReference::ContractReference;
use crate::traits::_TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::traits::TraitExternalData::TraitExternalData;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct POF_CE_PAM;

impl TraitPayOffFunction for POF_CE_PAM {
    fn new() -> Self {
        Self {}
    }
    fn eval(
        &self,
        _time: &PhantomIsoDatetimeW,
        _states: &StatesSpace,
        _contract_terms: &ContractTerms,
        _contract_structure: &Option<RelatedContracts>,
        _risk_factor_external_data: &Option<Box<dyn TraitExternalData>>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        0.0
    }
}
