use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::state_space::StateSpace::StateSpace;
use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_RRF_LAM;

impl TraitStateTransitionFunction for STF_RRF_LAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let status_date = states.status_date.clone().expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.clone().expect("nominalInterestRate should always be Some");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.clone().expect("interestCalculationBaseAmount should always be Some");
        let notional_principal = states.notional_principal.clone().expect("notionalPrincipal should always be Some");

        let fee_rate_m = model.fee_rate.clone().expect("fee rate should always be Some");
        let next_reset_rate_m = model.next_reset_rate.clone().expect("fee rate should always be Some");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.clone().value()),
            time_adjuster.shift_sc(time),
        );


        states.accrued_interest = AccruedInterest::new({
            states.accrued_interest.clone().unwrap().value() + nominal_interest_rate.value() * interest_calculation_base_amount.value() * time_from_last_event
        }).ok();

        states.fee_accrued = FeeAccrued::new({

            states.fee_accrued.clone().unwrap().value() + fee_rate_m.value() * notional_principal.value() * time_from_last_event
        }).ok();

        states.nominal_interest_rate = NominalInterestRate::new(next_reset_rate_m.value()).ok();
        states.status_date = Some(StatusDate::from(*time));
    }
}
