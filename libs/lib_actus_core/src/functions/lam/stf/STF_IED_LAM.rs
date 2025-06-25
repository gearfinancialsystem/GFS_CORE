use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::state_space::StateSpace::StateSpace;
use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use crate::types::isoDatetime::IsoDatetime;


#[allow(non_camel_case_types)]
pub struct STF_IED_LAM;

impl TraitStateTransitionFunction for STF_IED_LAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        // Create a mutable copy of the states to update
        let status_date = states.statusDate.expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominalInterestRate.expect("nominalInterestRate should always be Some");
        let interest_calculation_base_amount = states.interestCalculationBaseAmount.expect("interestCalculationBaseAmount should always be Some");
        let notional_principal = states.notionalPrincipal.expect("notionalPrincipal should always be Some");
        let fee_rate = model.feeRate.clone().expect("fee rate should always be Some");
        let contract_role = model.contractRole.clone().expect("contract role should always be Some");
        let notional_principal_m = model.notionalPrincipal.clone().expect("notionalPrincipal should always be Some");
        let nominal_interest_rate_m = model.nominalInterestRate.clone().expect("nominal interest rate should always be Some");
        let interest_calculation_base = model.interestCalculationBaseAmount.clone().expect("interestCalculationBaseAmount should always be Some");
        
        states.statusDate = Some(*time);
        states.notionalPrincipal = Some(contract_role.role_sign())
            * notional_principal_m;

        states.nominalInterestRate = Some(nominal_interest_rate_m);

        if InterestCalculationBase::NT == model.get_as("interestCalculationBase") {
            states.interestCalculationBaseAmount = states.notionalPrincipal;
        } else {
            states.interestCalculationBaseAmount = ContractRoleConvention::role_sign(model.get_as("contractRole"))
                * model.get_as::<f64>("interestCalculationBaseAmount");
        }

        if !CommonUtils::is_null(model.get_as("accruedInterest")) {
            states.accruedInterest = ContractRoleConvention::role_sign(model.get_as("contractRole"))
                * model.get_as::<f64>("accruedInterest");
        } else if !CommonUtils::is_null(model.get_as("cycleAnchorDateOfInterestPayment"))
            && model.get_as::<IsoDatetime>("cycleAnchorDateOfInterestPayment") < *time {
            let cycle_anchor_date = model.get_as::<IsoDatetime>("cycleAnchorDateOfInterestPayment");
            states.accruedInterest = day_counter.day_count_fraction(
                time_adjuster.shift_sc(&cycle_anchor_date),
                time_adjuster.shift_sc(time),
            ) * states.notionalPrincipal
                * states.interestCalculationBaseAmount;
        } else {
            states.accruedInterest = 0.0;
        }

    }
}
