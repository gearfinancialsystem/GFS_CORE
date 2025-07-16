use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_fees::FeeRate::FeeRate;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::traits::TraitOptionExt::TraitOptionExt;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_PI_LAX {
    pr_payment: f64,
}

impl STF_PI_LAX {
    pub fn new(pr_payment: f64) -> Self {
        STF_PI_LAX { pr_payment }
    }
}

impl TraitStateTransitionFunction for STF_PI_LAX {
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
        let role = &model.contract_role.clone().unwrap().role_sign();
        let redemption = role * self.pr_payment - role * (self.pr_payment.abs() - states.notional_principal.clone().itself_or(0.0).value().abs()    ).max(0.0);

        let a = states.notional_principal.clone();
        let b = a.itself_or_option(0.0);

        let status_date = states.status_date.clone().expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.clone().unwrap_or(NominalInterestRate::new(0.0).ok().unwrap());
        let interest_calculation_base_amount = states.interest_calculation_base_amount.clone().unwrap_or(InterestCalculationBaseAmount::new(0.0).ok().unwrap());

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.clone().value()),
            time_adjuster.shift_sc(time)
        );

        states.accrued_interest = AccruedInterest::new(states.accrued_interest.clone().map(|accrued_interest| {
            accrued_interest.value() + nominal_interest_rate.value() * interest_calculation_base_amount.value() * time_from_last_event
        }).unwrap()).ok();

        states.fee_accrued = FeeAccrued::new(states.fee_accrued.clone().map(|fee_accrued| {
            //let fee_rate = model.fee_rate.clone().unwrap_or(0.0);
            let fee_rate = model.fee_rate.itself_or(0.0);
            let notional_principal = states.notional_principal.itself_or(0.0);

            fee_accrued.value() + fee_rate.value() * notional_principal.value() * time_from_last_event
        }).unwrap()).ok();

        states.notional_principal =
            NotionalPrincipal::new(
            states.notional_principal.clone().map(|notional_principal| {
            notional_principal.value() + redemption
        }).unwrap()).ok();

        states.status_date = Some(StatusDate::from(*time));
    }
}
