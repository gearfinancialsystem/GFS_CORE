use crate::attributes::ContractTerms::ContractModel;

use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_events::traits::TraitPayOffFunction::TraitPayOffFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;

#[allow(non_camel_case_types)]
pub struct POF_MD_FUTUR;

impl TraitPayOffFunction for POF_MD_FUTUR {
    fn eval(
        &self,
        _time: &IsoDatetime,
        _states: &StatesSpace,
        _model: &ContractModel,
        risk_factor_model: &DataObserver,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        // Remplacer les appels à settlement_currency_fx_rate ou risk_factor_model par 1.0
        //let settlement_currency_fx_rate = 1.0;

        // Retourner 0.0 comme dans le code Java
        0.0
    }
}
