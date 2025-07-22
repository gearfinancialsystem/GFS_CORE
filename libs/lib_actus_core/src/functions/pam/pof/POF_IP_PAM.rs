use lib_actus_events::traits::TraitPayOffFunction::TraitPayOffFunction;
use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_types::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;

use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;


#[allow(non_camel_case_types)]
pub struct POF_IP_PAM;

impl TraitPayOffFunction for POF_IP_PAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StatesSpace,
        model: &ContractTerms,
        risk_factor_model: Option<&dyn TraitRiskFactorModel>,
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
            model,
            time,
            states
        );
        let res = settlement_currency_fx_rate * interest_scaling_multiplier.value() * (accrued_interest.value() + a) * nominal_interest_rate.value() * notional_principal.value();

        res
    }
}
