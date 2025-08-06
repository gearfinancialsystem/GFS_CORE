use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_optionality::PenaltyType::PenaltyType;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;

use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::attributes::ContractReference::ContractReference;

#[allow(non_camel_case_types)]
pub struct POF_PY_PAM;

impl TraitPayOffFunction for POF_PY_PAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StatesSpace,
        contract_terms: &ContractTerms,
        _contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<RiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let penalty_type = contract_terms.penalty_type.as_ref().expect("penaltyType should be Some");
        let contract_role = contract_terms.contract_role.as_ref().expect("contract role should be Some");

        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            contract_terms,
            time,
            states
        );



        match penalty_type {
            PenaltyType::A(_A) => {
                let penalty_rate = contract_terms.penalty_rate.as_ref().expect("penaltyRate should be Some");
                settlement_currency_fx_rate * contract_role.role_sign() * penalty_rate.value()
            }
            PenaltyType::N(_N) => {
                let penalty_rate = contract_terms.penalty_rate.as_ref().expect("penaltyRate should be Some");
                let status_date = states.status_date.as_ref().expect("status date should always exist");
                let notional_principal = states.notional_principal.as_ref().expect("notionalPrincipal should be Some");

                settlement_currency_fx_rate * contract_role.role_sign()
                    * day_counter.day_count_fraction(time_adjuster.shift_sc(&status_date.value()), time_adjuster.shift_sc(&time))
                * penalty_rate.value() * notional_principal.value()
            }
            _ => {
                let status_date = states.status_date.as_ref().expect("status date should always exist");
                let notional_principal = states.notional_principal.as_ref().expect("notionalPrincipal should always exist");
                let nominal_interest_rate = states.nominal_interest_rate.as_ref().expect("nominalInterestRate should be Some");
                //let market_object_code_of_rate_reset = contract_terms.marketObjectCodeOfRateReset.as_ref().expect("marketObjectCodeOfRateReset should be Some");
                let mut cbv = None;
                if let Some(rfm) = risk_factor_model {
                    cbv = rfm.state_at(
                        contract_terms.market_object_code_of_rate_reset.clone().unwrap().value(),
                        time,
                        states,
                        contract_terms,
                        true
                    );
                } else {
                    cbv = None
                }
                settlement_currency_fx_rate * contract_role.role_sign()
                    * day_counter.day_count_fraction(time_adjuster.shift_sc(&status_date.value()), time_adjuster.shift_sc(&time))
                    * notional_principal.value()
                    * 0.0f64.max(nominal_interest_rate.value() - cbv.unwrap())
            }
        }
    }
}
