use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;

use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::external::RiskFactorModel::RiskFactorModel;
use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use crate::attributes::ContractReference::ContractReference;
#[allow(non_camel_case_types)]
pub struct POF_IPFix_SWPPV;

impl TraitPayOffFunction for POF_IPFix_SWPPV {
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &StatesSpace,
        contract_terms: &ContractTerms,
        _contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<impl TraitRiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            contract_terms,
            time,
            states
        );
        let nominal_interest_rate = contract_terms.nominal_interest_rate.clone().expect("nominalInterestRate should always exist");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&states.status_date.clone().unwrap().value()),
            time_adjuster.shift_sc(time)
        );

        settlement_currency_fx_rate * (
            states.accrued_interest.clone().unwrap().value() +
                time_from_last_event *
                    nominal_interest_rate.value()  *
                    states.notional_principal.clone().unwrap().value()
        )
    }
}
