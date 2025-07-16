use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::traits::TraitOptionExt::TraitOptionExt;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_IPCB_LAM;

impl TraitStateTransitionFunction for STF_IPCB_LAM {
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

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.clone().value()),
            time_adjuster.shift_sc(time)
        );

        states.accrued_interest = AccruedInterest::new({
            states.accrued_interest.clone().unwrap().value() + nominal_interest_rate.value() * interest_calculation_base_amount.value() * time_from_last_event
        }).ok();

        
        
        states.fee_accrued = FeeAccrued::new({
            let fee_rate = {
                if model.fee_rate.is_none() {
                    0.0
                } else {
                    model.fee_rate.clone().unwrap().value()
                }
            };
            states.fee_accrued.clone().unwrap().value() + fee_rate * notional_principal.value() * time_from_last_event
        }).ok();
        

        states.status_date = Some(StatusDate::from(*time));
        states.interest_calculation_base_amount =InterestCalculationBaseAmount::new(states.notional_principal.clone().unwrap().value()).ok() ;
    }
}
