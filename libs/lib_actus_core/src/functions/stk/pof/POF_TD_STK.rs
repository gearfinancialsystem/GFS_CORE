use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;

use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_TD_STK;


impl TraitPayOffFunction for POF_TD_STK {
    fn eval(
        &self,
        _time: &IsoDatetime,
        _states: &StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let contract_role = model.contractRole.as_ref().expect("contract role should always be some");
        let quantity = model.quantity.expect("quantity should always be some");
        let price_at_termination_date = model.priceAtTerminationDate.expect("priceAtTermination date should always be some");

        1.0 * contract_role.role_sign() * quantity * price_at_termination_date

    }
}
