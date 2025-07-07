use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;


#[allow(non_camel_case_types)]
pub struct POF_PRD_LAM;

impl TraitPayOffFunction for POF_PRD_LAM {
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

        settlement_currency_fx_rate
            * model.clone().contractRole.unwrap().role_sign()
            * (-1.0)
            * (model.price_at_purchase_date.clone().unwrap() + states.accrued_interest.unwrap()
            + (day_counter.day_count_fraction(
                    time_adjuster.shift_sc(&states.status_date.unwrap()),
                    time_adjuster.shift_sc(time),
        ) * states.nominal_interest_rate.unwrap()
            * states.interest_calculation_base_amount.unwrap()))
    }
}
