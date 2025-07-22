use lib_actus_events::traits::TraitPayOffFunction::TraitPayOffFunction;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
// use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_types::types::IsoDatetime::IsoDatetime;


#[allow(non_camel_case_types)]
pub struct POF_AD_PAM;


impl TraitPayOffFunction for POF_AD_PAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StatesSpace,
        model: &ContractTerms,
        risk_factor_model: Option<&dyn lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        0.0
    }
}
