use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;

use lib_actus_types::types::IsoDatetime::IsoDatetime;

// use crate::attributes::ContractReference::ContractReference;
use crate::traits::_TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use lib_actus_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::traits::TraitExternalData::TraitExternalData;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct POF_IP_PAM;

impl TraitPayOffFunction for POF_IP_PAM {
    fn new() -> Self {
        Self {}
    }
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &StatesSpace,
        _contract_terms: &ContractTerms,
        _contract_structure: &Option<RelatedContracts>,
        risk_factor_external_data: &Option<Box<dyn TraitExternalData>>,
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
            time_adjuster.shift_sc(&status_date.to_phantom_type()),
            time_adjuster.shift_sc(&time)
        );
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_external_data,
            _contract_terms,
            time,
            states
        );
        let res = settlement_currency_fx_rate * interest_scaling_multiplier.value() * (accrued_interest.value() + a) * nominal_interest_rate.value() * notional_principal.value();

        res
    }
}
