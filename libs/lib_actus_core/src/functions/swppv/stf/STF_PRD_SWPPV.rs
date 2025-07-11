use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_settlement::DeliverySettlement::DeliverySettlement;
use crate::terms::grp_settlement::delivery_settlement::D::D;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;


#[allow(non_camel_case_types)]
pub struct STF_PRD_SWPPV;

impl TraitStateTransitionFunction for STF_PRD_SWPPV {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let status_date = states.status_date.expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.expect("nominalInterestRate should always be Some");
        let notional_principal = states.notional_principal.expect("notionalPrincipal should always be Some");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date),
            time_adjuster.shift_sc(time)
        );

        let model_nominal_interest_rate = model.nominal_interest_rate.unwrap_or(0.0);
        let delivery_settlement = model.delivery_settlement.as_ref().expect("deliverySettlement should always be Some");

        let interest_rate = match delivery_settlement {
            DeliverySettlement::D(D) => model_nominal_interest_rate,
            _ => model_nominal_interest_rate - nominal_interest_rate,
        };

        states.accrued_interest = states.accrued_interest.map(|mut accrued_interest| {
            accrued_interest += interest_rate * notional_principal * time_from_last_event;
            accrued_interest
        });

        states.accrued_interest2 = states.accrued_interest2.map(|mut accrued_interest2| {
            accrued_interest2 += (-1.0) * nominal_interest_rate * notional_principal * time_from_last_event;
            accrued_interest2
        });

        states.status_date = Some(StatusDate::from(*time));
    }
}
