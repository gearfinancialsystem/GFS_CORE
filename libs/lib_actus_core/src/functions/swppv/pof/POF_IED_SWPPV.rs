use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_events::traits::TraitPayOffFunction::TraitPayOffFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;
#[allow(non_camel_case_types)]
pub struct POF_IED_SWPPV;

impl TraitPayOffFunction for POF_IED_SWPPV {
    fn eval(
        &self,
        _time: &IsoDatetime,
        _states: &StatesSpace,
        _model: &ContractTerms,
        risk_factor_model: Option<&dyn TraitRiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        // Remplacer les appels à settlement_currency_fx_rate ou risk_factor_model par 1.0
        //let settlement_currency_fx_rate = 1.0;

        // Retourner 0.0 comme dans le code Java
        0.0
    }
}
