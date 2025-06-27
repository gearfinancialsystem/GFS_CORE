use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_IPFloat_SWPPV;

impl TraitPayOffFunction for POF_IPFloat_SWPPV {
    fn eval(
        &self,
        _time: &IsoDatetime,
        states: &StateSpace,
        _model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let settlement_currency_fx_rate = 1.0; // Remplacer par 1.0 comme demandé

        settlement_currency_fx_rate * (
            states.accruedInterest2.clone().unwrap() +
                (-1.0) *
                    states.lastInterestPeriod.clone().unwrap() *
                    states.nominalInterestRate.clone().unwrap() *
                    states.notionalPrincipal.clone().unwrap()
        )
    }
}
