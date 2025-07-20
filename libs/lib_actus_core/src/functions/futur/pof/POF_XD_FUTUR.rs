use crate::attributes::ContractTerms::ContractModel;

use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;

#[allow(non_camel_case_types)]
pub struct POF_XD_FUTUR;

impl TraitPayOffFunction for POF_XD_FUTUR {
    fn eval(
        &self,
        _time: &IsoDatetime,
        _states: &StateSpace,
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
