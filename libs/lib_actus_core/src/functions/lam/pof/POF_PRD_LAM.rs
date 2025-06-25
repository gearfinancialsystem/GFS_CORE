use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::isoDatetime::IsoDatetime;
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
            1.0
            * model.clone().contractRole.unwrap().role_sign()
            * (-1.0)
            * (model.priceAtPurchaseDate.clone().unwrap() + states.accruedInterest.unwrap()
            + (day_counter.day_count_fraction(
                    time_adjuster.shift_sc(&states.statusDate.unwrap()),
                    time_adjuster.shift_sc(time),
        ) * states.nominalInterestRate.unwrap()
            * states.interestCalculationBaseAmount.unwrap()))
    }
}
