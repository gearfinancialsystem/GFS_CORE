use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_IPFix_SWPPV;

impl TraitPayOffFunction for POF_IPFix_SWPPV {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            model,
            time,
            states
        );
        let nominal_interest_rate = model.nominal_interest_rate.clone().expect("nominalInterestRate should always exist");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&states.status_date.clone().unwrap().value()),
            time_adjuster.shift_sc(time)
        );

        settlement_currency_fx_rate * (
            states.accrued_interest.clone().unwrap().value() +
                time_from_last_event *
                    nominal_interest_rate.value()  *
                    states.notional_principal.clone().unwrap().value()
        )
    }
}
