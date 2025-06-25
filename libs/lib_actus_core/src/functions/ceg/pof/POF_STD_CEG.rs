use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_STD_CEG;

impl TraitPayOffFunction for POF_STD_CEG {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        // Remplacer les appels à settlement_currency_fx_rate ou risk_factor_model par 1.0
        let settlement_currency_fx_rate = 1.0;

        settlement_currency_fx_rate * (states.exerciseAmount.clone().unwrap() + states.feeAccrued.clone().unwrap())
    }
}
