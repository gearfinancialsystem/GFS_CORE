use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;

#[allow(non_camel_case_types)]
pub struct POF_IPCB_LAM;

impl TraitPayOffFunction for POF_IPCB_LAM {
    fn eval(
        &self,
        _time: &PhantomIsoDatetimeW,
        _states: &StatesSpace,
        _contract_terms: &ContractTerms,
        _contract_structure: &Option<Vec<ContractReference>>,
        _risk_factor_model: &Option<impl TraitRiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        0.0
    }
}
