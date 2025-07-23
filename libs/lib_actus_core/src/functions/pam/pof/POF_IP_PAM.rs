use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;

use crate::types::IsoDatetime::IsoDatetime;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::attributes::ContractReference::ContractReference;

#[allow(non_camel_case_types)]
pub struct POF_IP_PAM;

impl TraitPayOffFunction for POF_IP_PAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StatesSpace,
        _contract_terms: &ContractTerms,
        _contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<RiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let interest_scaling_multiplier = states.interest_scaling_multiplier.as_ref().expect("interestScalingMultiplier should always be some");
        let accrued_interest = states.accrued_interest.as_ref().expect("accruedInterest should always be some");
        let nominal_interest_rate = states.nominal_interest_rate.as_ref().expect("nominalInterestRate should always be some");
        let notional_principal = states.notional_principal.as_ref().expect("notionalPrincipal should always be some");
        let status_date = states.status_date.as_ref().expect("status date should always be some");
        let a = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.value()),
            time_adjuster.shift_sc(&time)
        );
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            _contract_terms,
            time,
            states
        );
        let res = settlement_currency_fx_rate * interest_scaling_multiplier.value() * (accrued_interest.value() + a) * nominal_interest_rate.value() * notional_principal.value();

        res
    }
}
